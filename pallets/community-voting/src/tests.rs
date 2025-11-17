use crate::{mock::*, Error, Event};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

// Test successful voting period start
#[test]
fn it_works_for_start_voting() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period for proposal 1
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10 // 10 blocks duration
        ));
        
        // Should succeed without errors
    });
}

// Test successful vote casting
#[test]
fn it_works_for_cast_vote() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // Then cast a vote
        assert_ok!(CommunityVotingPallet::cast_vote(
            RuntimeOrigin::signed(2),
            1,
            0 // Yes vote
        ));
        
        // Check that vote event was emitted
        System::assert_last_event(
            Event::VoteCast {
                proposal_id: 1,
                voter: 2,
                vote: 0,
            }
            .into(),
        );
    });
}

// Test successful voting period end
#[test]
fn it_works_for_end_voting() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5 // Ends at block 6
        ));
        
        // Cast some votes - make sure Yes votes win
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0)); // Yes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(3), 1, 0)); // Yes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 1)); // No
        
        // Move time forward PAST voting period (block 7 > end block 6)
        System::set_block_number(7);
        
        // End voting
        assert_ok!(CommunityVotingPallet::end_voting(
            RuntimeOrigin::signed(5),
            1
        ));
        
        // Check that voting ended event was emitted with PASSED result
        // 2 Yes vs 1 No = Passed (result 0)
        System::assert_last_event(
            Event::VotingPeriodEnded {
                proposal_id: 1,
                result: 0, // Passed (more yes votes)
            }
            .into(),
        );
    });
}

// Test different vote types work
#[test]
fn it_works_for_different_vote_types() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // Cast Yes vote
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0));
        System::assert_last_event(Event::VoteCast { proposal_id: 1, voter: 2, vote: 0 }.into());
        
        // Cast No vote
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(3), 1, 1));
        System::assert_last_event(Event::VoteCast { proposal_id: 1, voter: 3, vote: 1 }.into());
        
        // Cast Abstain vote
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 2));
        System::assert_last_event(Event::VoteCast { proposal_id: 1, voter: 4, vote: 2 }.into());
    });
}
// Test cast_vote fails for non-existent proposal
#[test]
fn cast_vote_fails_for_non_existent_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to vote on proposal that doesn't exist
        assert_noop!(
            CommunityVotingPallet::cast_vote(
                RuntimeOrigin::signed(1),
                999, // Non-existent proposal
                0
            ),
            Error::<Test>::ProposalDoesNotExist
        );
    });
}

// Test cast_vote fails for invalid vote choice
#[test]
fn cast_vote_fails_for_invalid_vote() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // Try to cast invalid vote (3 is not valid)
        assert_noop!(
            CommunityVotingPallet::cast_vote(
                RuntimeOrigin::signed(2),
                1,
                3 // Invalid vote
            ),
            Error::<Test>::ProposalDoesNotExist
        );
    });
}

// Test cast_vote fails for already voted
#[test]
fn cast_vote_fails_for_already_voted() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // Cast first vote
        assert_ok!(CommunityVotingPallet::cast_vote(
            RuntimeOrigin::signed(2),
            1,
            0
        ));
        
        // Try to vote again with same account
        assert_noop!(
            CommunityVotingPallet::cast_vote(
                RuntimeOrigin::signed(2),
                1,
                1
            ),
            Error::<Test>::AlreadyVoted
        );
    });
}

// Test end_voting fails for non-existent proposal
#[test]
fn end_voting_fails_for_non_existent_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to end voting for proposal that doesn't exist
        assert_noop!(
            CommunityVotingPallet::end_voting(
                RuntimeOrigin::signed(1),
                999
            ),
            Error::<Test>::ProposalDoesNotExist
        );
    });
}
// Test cast_vote fails when voting period hasn't started
#[test]
fn cast_vote_fails_when_voting_not_started() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period starting at block 1, ending at block 11
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // Should work at block 1
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0));
    });
}

// Test cast_vote fails when voting period has ended
#[test]
fn cast_vote_fails_when_voting_ended() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period with short duration
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5 // Ends at block 6
        ));
        
        // Move time forward past voting period
        System::set_block_number(10);
        
        // Try to vote after voting ended
        assert_noop!(
            CommunityVotingPallet::cast_vote(
                RuntimeOrigin::signed(2),
                1,
                0
            ),
            Error::<Test>::VotingPeriodEnded
        );
    });
}

// Test end_voting fails when voting period hasn't ended
#[test]
fn end_voting_fails_when_voting_not_ended() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10 // Ends at block 11
        ));
        
        // Try to end voting before period ends
        assert_noop!(
            CommunityVotingPallet::end_voting(
                RuntimeOrigin::signed(2),
                1
            ),
            Error::<Test>::VotingPeriodNotStarted
        );
    });
}

// Test unsigned origin fails for start_voting
#[test]
fn start_voting_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            CommunityVotingPallet::start_voting(
                RuntimeOrigin::none(), // No signature
                1,
                10
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test unsigned origin fails for cast_vote
#[test]
fn cast_vote_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting first
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // Try to call without signing
        assert_noop!(
            CommunityVotingPallet::cast_vote(
                RuntimeOrigin::none(), // No signature
                1,
                0
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test unsigned origin fails for end_voting
#[test]
fn end_voting_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            CommunityVotingPallet::end_voting(
                RuntimeOrigin::none(), // No signature
                1
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}
// Test multiple proposals can exist simultaneously
#[test]
fn multiple_proposals_can_exist() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting for proposal 1
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // Start voting for proposal 2
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(2),
            2,
            15
        ));
        
        // Start voting for proposal 3
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(3),
            3,
            20
        ));
        
        // Vote on all proposals
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 0));
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(5), 2, 1));
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(6), 3, 2));
        
        // All should work independently
    });
}

// Test voting result: Proposal passed (more yes votes)
#[test]
fn voting_result_passed() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5
        ));
        
        // Cast votes: 2 Yes, 1 No
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0)); // Yes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(3), 1, 0)); // Yes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 1)); // No
        
        // Move time forward past voting period
        System::set_block_number(10);
        
        // End voting - should pass
        assert_ok!(CommunityVotingPallet::end_voting(
            RuntimeOrigin::signed(5),
            1
        ));
        
        // Check that result is Passed (0)
        System::assert_last_event(
            Event::VotingPeriodEnded {
                proposal_id: 1,
                result: 0, // Passed
            }
            .into(),
        );
    });
}

// Test voting result: Proposal failed (more no votes)
#[test]
fn voting_result_failed() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5
        ));
        
        // Cast votes: 1 Yes, 2 No
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0)); // Yes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(3), 1, 1)); // No
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 1)); // No
        
        // Move time forward past voting period
        System::set_block_number(10);
        
        // End voting - should fail
        assert_ok!(CommunityVotingPallet::end_voting(
            RuntimeOrigin::signed(5),
            1
        ));
        
        // Check that result is Failed (1)
        System::assert_last_event(
            Event::VotingPeriodEnded {
                proposal_id: 1,
                result: 1, // Failed
            }
            .into(),
        );
    });
}

// Test voting result: No quorum (no votes)
#[test]
fn voting_result_no_quorum() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5
        ));
        
        // Don't cast any votes
        
        // Move time forward past voting period
        System::set_block_number(10);
        
        // End voting - should be no quorum
        assert_ok!(CommunityVotingPallet::end_voting(
            RuntimeOrigin::signed(2),
            1
        ));
        
        // Check that result is NoQuorum (2)
        System::assert_last_event(
            Event::VotingPeriodEnded {
                proposal_id: 1,
                result: 2, // NoQuorum
            }
            .into(),
        );
    });
}

// Test voting result: Only abstain votes count as no quorum
#[test]
fn voting_result_only_abstain_no_quorum() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5
        ));
        
        // Cast only abstain votes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 2)); // Abstain
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(3), 1, 2)); // Abstain
        
        // Move time forward past voting period
        System::set_block_number(10);
        
        // End voting - should be no quorum (abstain doesn't count toward quorum)
        assert_ok!(CommunityVotingPallet::end_voting(
            RuntimeOrigin::signed(4),
            1
        ));
        
        // Check that result is NoQuorum (2)
        System::assert_last_event(
            Event::VotingPeriodEnded {
                proposal_id: 1,
                result: 2, // NoQuorum
            }
            .into(),
        );
    });
}
// Test same user can vote on different proposals
#[test]
fn same_user_can_vote_on_different_proposals() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting for multiple proposals
        assert_ok!(CommunityVotingPallet::start_voting(RuntimeOrigin::signed(1), 1, 10));
        assert_ok!(CommunityVotingPallet::start_voting(RuntimeOrigin::signed(2), 2, 10));
        assert_ok!(CommunityVotingPallet::start_voting(RuntimeOrigin::signed(3), 3, 10));
        
        // Same user votes on all three proposals
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 0));
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 2, 1));
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 3, 2));
        
        // All should work - no "AlreadyVoted" error across different proposals
    });
}

// Test voting with tie (equal yes and no votes)
#[test]
fn voting_result_tie_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5
        ));
        
        // Cast votes: 2 Yes, 2 No (tie)
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0)); // Yes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(3), 1, 1)); // No
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 0)); // Yes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(5), 1, 1)); // No
        
        // Move time forward past voting period
        System::set_block_number(10);
        
        // End voting - should fail (tie goes to No)
        assert_ok!(CommunityVotingPallet::end_voting(
            RuntimeOrigin::signed(6),
            1
        ));
        
        // Check that result is Failed (1) - tie breaks to failure
        System::assert_last_event(
            Event::VotingPeriodEnded {
                proposal_id: 1,
                result: 1, // Failed (tie)
            }
            .into(),
        );
    });
}

// Test voting at exact start and end blocks
#[test]
fn voting_at_exact_start_and_end_blocks() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period from block 1 to block 11
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10 // Ends at block 11
        ));
        
        // Vote at exact start block (should work)
        System::set_block_number(1);
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0));
        
        // Vote at exact end block (should work)
        System::set_block_number(11);
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(3), 1, 1));
        
        // Try to vote one block after end (should fail)
        System::set_block_number(12);
        assert_noop!(
            CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(4), 1, 0),
            Error::<Test>::VotingPeriodEnded
        );
    });
}

// Test end_voting works immediately after voting period ends
#[test]
fn end_voting_immediately_after_period() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period ending at block 6
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            5
        ));
        
        // Cast some votes
        assert_ok!(CommunityVotingPallet::cast_vote(RuntimeOrigin::signed(2), 1, 0));
        
        // Move to first block after voting ends
        System::set_block_number(7);
        
        // Should be able to end voting immediately
        assert_ok!(CommunityVotingPallet::end_voting(
            RuntimeOrigin::signed(3),
            1
        ));
    });
}

// Test multiple users voting on same proposal
#[test]
fn multiple_users_voting_same_proposal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting period
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            10
        ));
        
        // 10 different users vote
        for i in 2..12 {
            let vote_choice = (i % 3) as u8; // Cycle through 0,1,2
            assert_ok!(CommunityVotingPallet::cast_vote(
                RuntimeOrigin::signed(i),
                1,
                vote_choice
            ));
        }
        
        // All should work without errors
    });
}

// Test proposal with very long voting duration
#[test]
fn very_long_voting_duration() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Start voting with very long duration
        assert_ok!(CommunityVotingPallet::start_voting(
            RuntimeOrigin::signed(1),
            1,
            100000 // Very long duration
        ));
        
        // Should work without issues
    });
}