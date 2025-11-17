use crate::{mock::*, Error, Event};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

// Test successful budget proposal creation
#[test]
fn it_works_for_create_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create budget proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Community development project".to_vec()
        ));
        
        // Check that the event was emitted correctly
        System::assert_last_event(
            Event::BudgetProposalCreated {
                proposal_id: 0,
                creator: 1,
                amount: 1000,
                purpose: BoundedVec::try_from(b"Community development project".to_vec()).unwrap(),
            }
            .into(),
        );
    });
}

// Test successful proposal approval
#[test]
fn it_works_for_approve_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            500,
            b"Infrastructure upgrade".to_vec()
        ));
        
        // Then approve it
        assert_ok!(BudgetProposalPallet::approve_proposal(
            RuntimeOrigin::signed(2), // Different user approves
            0
        ));
        
        // Check that approval event was emitted
        System::assert_last_event(
            Event::BudgetProposalApproved {
                proposal_id: 0,
                approver: 2,
            }
            .into(),
        );
    });
}

// Test successful proposal rejection
#[test]
fn it_works_for_reject_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            2000,
            b"Marketing campaign".to_vec()
        ));
        
        // Then reject it
        assert_ok!(BudgetProposalPallet::reject_proposal(
            RuntimeOrigin::signed(3), // Different user rejects
            0,
            b"Budget too high".to_vec()
        ));
        
        // Check that rejection event was emitted
        System::assert_last_event(
            Event::BudgetProposalRejected {
                proposal_id: 0,
                rejecter: 3,
                reason: BoundedVec::try_from(b"Budget too high".to_vec()).unwrap(),
            }
            .into(),
        );
    });
}
// Test empty purpose is rejected
#[test]
fn create_proposal_fails_for_empty_purpose() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to create proposal with empty purpose
        assert_noop!(
            BudgetProposalPallet::create_proposal(
                RuntimeOrigin::signed(1),
                1000,
                b"".to_vec() // Empty purpose
            ),
            Error::<Test>::InvalidPurpose
        );
    });
}

// Test purpose that is too long is rejected
#[test]
fn create_proposal_fails_for_too_long_purpose() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create purpose that exceeds 200 bytes
        let long_purpose = b"A".repeat(201);
        
        assert_noop!(
            BudgetProposalPallet::create_proposal(
                RuntimeOrigin::signed(1),
                1000,
                long_purpose
            ),
            Error::<Test>::InvalidPurpose
        );
    });
}

// Test empty reason is rejected for rejection
#[test]
fn reject_proposal_fails_for_empty_reason() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Some purpose".to_vec()
        ));
        
        // Try to reject with empty reason
        assert_noop!(
            BudgetProposalPallet::reject_proposal(
                RuntimeOrigin::signed(2),
                0,
                b"".to_vec() // Empty reason
            ),
            Error::<Test>::InvalidReason
        );
    });
}

// Test reason that is too long is rejected
#[test]
fn reject_proposal_fails_for_too_long_reason() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Some purpose".to_vec()
        ));
        
        // Create reason that exceeds 100 bytes
        let long_reason = b"X".repeat(101);
        
        assert_noop!(
            BudgetProposalPallet::reject_proposal(
                RuntimeOrigin::signed(2),
                0,
                long_reason
            ),
            Error::<Test>::InvalidReason
        );
    });
}
// Test unsigned origin fails for create_proposal
#[test]
fn create_proposal_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            BudgetProposalPallet::create_proposal(
                RuntimeOrigin::none(), // No signature
                1000,
                b"Some purpose".to_vec()
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test unsigned origin fails for approve_proposal
#[test]
fn approve_proposal_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            BudgetProposalPallet::approve_proposal(
                RuntimeOrigin::none(), // No signature
                0
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test unsigned origin fails for reject_proposal
#[test]
fn reject_proposal_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            BudgetProposalPallet::reject_proposal(
                RuntimeOrigin::none(), // No signature
                0,
                b"Some reason".to_vec()
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test approve_proposal fails for non-existent proposal
#[test]
fn approve_proposal_fails_for_non_existent_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to approve proposal that doesn't exist
        assert_noop!(
            BudgetProposalPallet::approve_proposal(
                RuntimeOrigin::signed(1),
                999 // Non-existent proposal ID
            ),
            Error::<Test>::ProposalDoesNotExist
        );
    });
}

// Test reject_proposal fails for non-existent proposal
#[test]
fn reject_proposal_fails_for_non_existent_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to reject proposal that doesn't exist
        assert_noop!(
            BudgetProposalPallet::reject_proposal(
                RuntimeOrigin::signed(1),
                999, // Non-existent proposal ID
                b"Some reason".to_vec()
            ),
            Error::<Test>::ProposalDoesNotExist
        );
    });
}
// Test cannot approve already approved proposal
#[test]
fn approve_proposal_fails_for_already_approved() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create and approve a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Some purpose".to_vec()
        ));
        
        assert_ok!(BudgetProposalPallet::approve_proposal(
            RuntimeOrigin::signed(2),
            0
        ));
        
        // Try to approve again
        assert_noop!(
            BudgetProposalPallet::approve_proposal(
                RuntimeOrigin::signed(3),
                0
            ),
            Error::<Test>::ProposalNotPending
        );
    });
}

// Test cannot reject already rejected proposal
#[test]
fn reject_proposal_fails_for_already_rejected() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create and reject a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Some purpose".to_vec()
        ));
        
        assert_ok!(BudgetProposalPallet::reject_proposal(
            RuntimeOrigin::signed(2),
            0,
            b"Budget issue".to_vec()
        ));
        
        // Try to reject again
        assert_noop!(
            BudgetProposalPallet::reject_proposal(
                RuntimeOrigin::signed(3),
                0,
                b"Another reason".to_vec()
            ),
            Error::<Test>::ProposalNotPending
        );
    });
}

// Test cannot reject already approved proposal
#[test]
fn reject_proposal_fails_for_approved_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create and approve a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Some purpose".to_vec()
        ));
        
        assert_ok!(BudgetProposalPallet::approve_proposal(
            RuntimeOrigin::signed(2),
            0
        ));
        
        // Try to reject approved proposal
        assert_noop!(
            BudgetProposalPallet::reject_proposal(
                RuntimeOrigin::signed(3),
                0,
                b"Too late".to_vec()
            ),
            Error::<Test>::ProposalNotPending
        );
    });
}

// Test cannot approve already rejected proposal
#[test]
fn approve_proposal_fails_for_rejected_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create and reject a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Some purpose".to_vec()
        ));
        
        assert_ok!(BudgetProposalPallet::reject_proposal(
            RuntimeOrigin::signed(2),
            0,
            b"Not feasible".to_vec()
        ));
        
        // Try to approve rejected proposal
        assert_noop!(
            BudgetProposalPallet::approve_proposal(
                RuntimeOrigin::signed(3),
                0
            ),
            Error::<Test>::ProposalNotPending
        );
    });
}
// Test multiple proposals have sequential IDs
#[test]
fn multiple_proposals_have_sequential_ids() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create first proposal - should get ID 0
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"First proposal".to_vec()
        ));
        
        // Create second proposal - should get ID 1  
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(2),
            2000,
            b"Second proposal".to_vec()
        ));
        
        // Create third proposal - should get ID 2
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(3),
            3000,
            b"Third proposal".to_vec()
        ));
        
        // Verify all proposals can be approved with correct IDs
        assert_ok!(BudgetProposalPallet::approve_proposal(RuntimeOrigin::signed(4), 0));
        assert_ok!(BudgetProposalPallet::approve_proposal(RuntimeOrigin::signed(5), 1));
        assert_ok!(BudgetProposalPallet::approve_proposal(RuntimeOrigin::signed(6), 2));
        
        // Proposal 3 should not exist
        assert_noop!(
            BudgetProposalPallet::approve_proposal(RuntimeOrigin::signed(7), 3),
            Error::<Test>::ProposalDoesNotExist
        );
    });
}

// Test different users can create proposals
#[test]
fn different_users_can_create_proposals() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // User 1 creates proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            500,
            b"User 1 project".to_vec()
        ));
        
        // User 2 creates proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(2),
            1500,
            b"User 2 project".to_vec()
        ));
        
        // User 3 creates proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(3),
            2500,
            b"User 3 project".to_vec()
        ));
        
        // All should have different IDs and work
        assert_ok!(BudgetProposalPallet::approve_proposal(RuntimeOrigin::signed(4), 0));
        assert_ok!(BudgetProposalPallet::reject_proposal(RuntimeOrigin::signed(5), 1, b"Budget high".to_vec()));
        assert_ok!(BudgetProposalPallet::approve_proposal(RuntimeOrigin::signed(6), 2));
    });
}

// Test proposal with maximum allowed purpose length
#[test]
fn it_works_for_max_length_purpose() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create purpose with exactly 200 bytes (maximum)
        let max_purpose = b"A".repeat(200);
        
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            max_purpose
        ));
        
        // Should work and emit event
        System::assert_last_event(
            Event::BudgetProposalCreated {
                proposal_id: 0,
                creator: 1,
                amount: 1000,
                purpose: BoundedVec::try_from(b"A".repeat(200)).unwrap(),
            }
            .into(),
        );
    });
}

// Test rejection with maximum allowed reason length
#[test]
fn it_works_for_max_length_reason() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create a proposal
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            1000,
            b"Some purpose".to_vec()
        ));
        
        // Create reason with exactly 100 bytes (maximum)
        let max_reason = b"X".repeat(100);
        
        assert_ok!(BudgetProposalPallet::reject_proposal(
            RuntimeOrigin::signed(2),
            0,
            max_reason
        ));
        
        // Should work and emit event
        System::assert_last_event(
            Event::BudgetProposalRejected {
                proposal_id: 0,
                rejecter: 2,
                reason: BoundedVec::try_from(b"X".repeat(100)).unwrap(),
            }
            .into(),
        );
    });
}

// Test large amount values
#[test]
fn it_works_for_large_amounts() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create proposal with very large amount
        assert_ok!(BudgetProposalPallet::create_proposal(
            RuntimeOrigin::signed(1),
            u128::MAX, // Maximum u128 value
            b"Large budget project".to_vec()
        ));
        
        // Should work and emit event with correct amount
        System::assert_last_event(
            Event::BudgetProposalCreated {
                proposal_id: 0,
                creator: 1,
                amount: u128::MAX,
                purpose: BoundedVec::try_from(b"Large budget project".to_vec()).unwrap(),
            }
            .into(),
        );
    });
}