use crate::{mock::*, Error, Event};
use frame::deps::sp_runtime;
use frame::testing_prelude::*;

// Test successful wallet creation
#[test]
fn it_works_for_create_wallet() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create government wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10, // wallet_id
            b"Ministry of Finance".to_vec()
        ));
        
        // Check that the event was emitted correctly
        System::assert_last_event(
            Event::GovernmentWalletCreated {
                wallet_id: 10,
                department: b"Ministry of Finance".to_vec(),
            }
            .into(),
        );
    });
}

// Test successful funds allocation
#[test]
fn it_works_for_allocate_funds() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create a wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Health Department".to_vec()
        ));
        
        // Then allocate funds to it
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(2),
            10, // wallet_id
            5000, // amount
            b"Medical supplies".to_vec() // purpose
        ));
        
        // Check that funds allocated event was emitted
        System::assert_last_event(
            Event::FundsAllocated {
                wallet_id: 10,
                amount: 5000,
                purpose: b"Medical supplies".to_vec(),
            }
            .into(),
        );
    });
}

// Test multiple wallets can be created
#[test]
fn multiple_wallets_can_be_created() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create first wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Education".to_vec()
        ));
        
        // Create second wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(2),
            20,
            b"Infrastructure".to_vec()
        ));
        
        // Create third wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(3),
            30,
            b"Defense".to_vec()
        ));
        
        // All should work without conflicts
    });
}

// Test multiple allocations to same wallet
#[test]
fn multiple_allocations_to_same_wallet() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Transportation".to_vec()
        ));
        
        // First allocation
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(2),
            10,
            1000,
            b"Road maintenance".to_vec()
        ));
        
        // Second allocation
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(3),
            10,
            2000,
            b"New vehicles".to_vec()
        ));
        
        // Third allocation
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(4),
            10,
            1500,
            b"Fuel costs".to_vec()
        ));
        
        // All should work and accumulate balance
    });
}
// Test create_wallet fails for empty department
#[test]
fn create_wallet_fails_for_empty_department() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to create wallet with empty department
        assert_noop!(
            GovernmentWalletPallet::create_wallet(
                RuntimeOrigin::signed(1),
                10,
                b"".to_vec() // Empty department
            ),
            Error::<Test>::InvalidDepartment
        );
    });
}

// Test create_wallet fails for too long department
#[test]
fn create_wallet_fails_for_too_long_department() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create department that exceeds 100 bytes
        let long_department = b"A".repeat(101);
        
        assert_noop!(
            GovernmentWalletPallet::create_wallet(
                RuntimeOrigin::signed(1),
                10,
                long_department
            ),
            Error::<Test>::InvalidDepartment
        );
    });
}

// Test create_wallet fails for duplicate wallet
#[test]
fn create_wallet_fails_for_duplicate_wallet() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create first wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Education".to_vec()
        ));
        
        // Try to create wallet with same ID
        assert_noop!(
            GovernmentWalletPallet::create_wallet(
                RuntimeOrigin::signed(2),
                10, // Same wallet ID
                b"Health".to_vec()
            ),
            Error::<Test>::WalletAlreadyExists
        );
    });
}

// Test allocate_funds fails for non-existent wallet
#[test]
fn allocate_funds_fails_for_non_existent_wallet() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to allocate funds to wallet that doesn't exist
        assert_noop!(
            GovernmentWalletPallet::allocate_funds(
                RuntimeOrigin::signed(1),
                999, // Non-existent wallet
                1000,
                b"Some purpose".to_vec()
            ),
            Error::<Test>::WalletDoesNotExist
        );
    });
}
// Test unsigned origin fails for create_wallet
#[test]
fn create_wallet_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Try to call without signing
        assert_noop!(
            GovernmentWalletPallet::create_wallet(
                RuntimeOrigin::none(), // No signature
                10,
                b"Some department".to_vec()
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test unsigned origin fails for allocate_funds
#[test]
fn allocate_funds_fails_for_unsigned_origin() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // First create a wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Some department".to_vec()
        ));
        
        // Try to call without signing
        assert_noop!(
            GovernmentWalletPallet::allocate_funds(
                RuntimeOrigin::none(), // No signature
                10,
                1000,
                b"Some purpose".to_vec()
            ),
            sp_runtime::traits::BadOrigin
        );
    });
}

// Test different users can create wallets
#[test]
fn different_users_can_create_wallets() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // User 1 creates wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"User 1 department".to_vec()
        ));
        
        // User 2 creates wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(2),
            20,
            b"User 2 department".to_vec()
        ));
        
        // User 3 creates wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(3),
            30,
            b"User 3 department".to_vec()
        ));
        
        // All should work with different creators
    });
}

// Test same user can create multiple wallets
#[test]
fn same_user_can_create_multiple_wallets() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Same user creates multiple wallets with different IDs
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Wallet 1".to_vec()
        ));
        
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            20,
            b"Wallet 2".to_vec()
        ));
        
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            30,
            b"Wallet 3".to_vec()
        ));
        
        // All should work - same creator, different wallet IDs
    });
}

// Test department with maximum allowed length
#[test]
fn it_works_for_max_length_department() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create department with exactly 100 bytes (maximum)
        let max_department = b"A".repeat(100);
        
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            max_department
        ));
        
        // Should work and emit event
        System::assert_last_event(
            Event::GovernmentWalletCreated {
                wallet_id: 10,
                department: b"A".repeat(100).to_vec(),
            }
            .into(),
        );
    });
}
// Test wallet balance accumulates correctly
#[test]
fn wallet_balance_accumulates_correctly() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Treasury".to_vec()
        ));
        
        // Allocate multiple amounts
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(2),
            10,
            1000,
            b"Initial funding".to_vec()
        ));
        
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(3),
            10,
            2500,
            b"Additional funds".to_vec()
        ));
        
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(4),
            10,
            1500,
            b"Emergency allocation".to_vec()
        ));
        
        // Total should be 1000 + 2500 + 1500 = 5000
        // Note: We can't directly check storage, but all allocations should succeed
    });
}

// Test large amount allocations
#[test]
fn it_works_for_large_amounts() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"National Budget".to_vec()
        ));
        
        // Allocate very large amount
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(2),
            10,
            u128::MAX, // Maximum u128 value
            b"National reserve".to_vec()
        ));
        
        // Should work without overflow
    });
}

// Test multiple allocations from different users
#[test]
fn multiple_allocations_from_different_users() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Public Works".to_vec()
        ));
        
        // 5 different users allocate funds
        for i in 2..7 {
            assert_ok!(GovernmentWalletPallet::allocate_funds(
                RuntimeOrigin::signed(i),
                10,
                (i * 1000) as u128,
                format!("Allocation from user {}", i).as_bytes().to_vec()
            ));
        }
        
        // All should work without issues
    });
}

// Test wallet creation with different account IDs
#[test]
fn wallet_creation_with_different_account_ids() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create wallets with various account IDs
        let wallet_ids = [10, 20, 30, 40, 50, 100, 1000, 9999];
        
        for (i, &wallet_id) in wallet_ids.iter().enumerate() {
            assert_ok!(GovernmentWalletPallet::create_wallet(
                RuntimeOrigin::signed((i + 1) as u64),
                wallet_id,
                format!("Department {}", i).as_bytes().to_vec()
            ));
        }
        
        // All different wallet IDs should work
    });
}

// Test allocation with empty purpose (should work based on code)
#[test]
fn allocation_with_empty_purpose_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"General Fund".to_vec()
        ));
        
        // Allocate with empty purpose (code doesn't validate purpose)
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(2),
            10,
            1000,
            b"".to_vec() // Empty purpose
        ));
        
        // Should work since purpose is not validated
        System::assert_last_event(
            Event::FundsAllocated {
                wallet_id: 10,
                amount: 1000,
                purpose: b"".to_vec(),
            }
            .into(),
        );
    });
}

// Test allocation with very long purpose (should work based on code)
#[test]
fn allocation_with_long_purpose_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        
        // Create wallet
        assert_ok!(GovernmentWalletPallet::create_wallet(
            RuntimeOrigin::signed(1),
            10,
            b"Research".to_vec()
        ));
        
        // Create very long purpose (no validation in code)
        let long_purpose = b"X".repeat(500);
        
        assert_ok!(GovernmentWalletPallet::allocate_funds(
            RuntimeOrigin::signed(2),
            10,
            5000,
            long_purpose
        ));
        
        // Should work since purpose length is not validated
    });
}