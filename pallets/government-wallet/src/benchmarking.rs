#![cfg(feature = "runtime-benchmarks")]

use super::{Pallet as GovernmentWalletPallet, *};
use frame::deps::frame_support::assert_ok;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as GovernmentWalletPallet;
    use frame_system::RawOrigin;

    #[benchmark]
    fn create_wallet() {
        let creator: T::AccountId = whitelisted_caller();
        let wallet_id: T::AccountId = whitelisted_caller(); // Different account as wallet ID
        
        // Setup: Prepare wallet parameters
        let department = b"Ministry of Finance".to_vec();
        
        #[extrinsic_call]
        create_wallet(
            RawOrigin::Signed(creator.clone()), 
            wallet_id.clone(),
            department
        );

        // Verification: Check that wallet was created and balance initialized
        assert!(GovernmentWallets::<T>::contains_key(&wallet_id));
        
        // WalletBalances use ValueQuery, so check if balance is zero
        let balance = WalletBalances::<T>::get(&wallet_id);
        assert_eq!(balance, 0u128);
    }
        #[benchmark]
    fn allocate_funds() {
        let creator: T::AccountId = whitelisted_caller();
        let allocator: T::AccountId = whitelisted_caller(); // Different account
        let wallet_id: T::AccountId = whitelisted_caller(); // Wallet account
        
        // Setup: First create a wallet
        assert_ok!(GovernmentWalletPallet::<T>::create_wallet(
            RawOrigin::Signed(creator.clone()).into(),
            wallet_id.clone(),
            b"Health Department".to_vec()
        ));

        // Allocate funds with normal parameters
        let amount = 5000u128;
        let purpose = b"Medical supplies".to_vec();

        #[extrinsic_call]
        allocate_funds(
            RawOrigin::Signed(allocator.clone()), 
            wallet_id.clone(),
            amount,
            purpose
        );

        // Verification: Check that balance was updated
        let balance = WalletBalances::<T>::get(&wallet_id);
        assert_eq!(balance, amount); // Should be exactly the allocated amount
    }
    #[benchmark]
    fn create_wallet_with_max_department() {
        let creator: T::AccountId = whitelisted_caller();
        let wallet_id: T::AccountId = whitelisted_caller(); // Different account as wallet ID
        
        // Setup: Prepare maximum size department (100 bytes)
        let department = b"A".repeat(100); // Maximum allowed size

        #[extrinsic_call]
        create_wallet(
            RawOrigin::Signed(creator.clone()), 
            wallet_id.clone(),
            department
        );

        // Verification: Check that wallet was created with max department
        assert!(GovernmentWallets::<T>::contains_key(&wallet_id));
        
        let wallet_details = GovernmentWallets::<T>::get(&wallet_id)
            .expect("Wallet should exist");
        assert_eq!(wallet_details.department.len(), 100); // Verify max size stored
        
        let balance = WalletBalances::<T>::get(&wallet_id);
        assert_eq!(balance, 0u128);
    }
        #[benchmark]
    fn allocate_funds_max_amount() {
        let creator: T::AccountId = whitelisted_caller();
        let allocator: T::AccountId = whitelisted_caller(); // Different account
        let wallet_id: T::AccountId = whitelisted_caller(); // Wallet account
        
        // Setup: First create a wallet
        assert_ok!(GovernmentWalletPallet::<T>::create_wallet(
            RawOrigin::Signed(creator.clone()).into(),
            wallet_id.clone(),
            b"National Budget".to_vec()
        ));

        // Allocate maximum possible amount
        let amount = u128::MAX;
        let purpose = b"National reserve".to_vec();

        #[extrinsic_call]
        allocate_funds(
            RawOrigin::Signed(allocator.clone()), 
            wallet_id.clone(),
            amount,
            purpose
        );

        // Verification: Check that balance was updated to max amount
        let balance = WalletBalances::<T>::get(&wallet_id);
        assert_eq!(balance, u128::MAX); // Should be exactly the maximum amount
    }
    impl_benchmark_test_suite!(
        GovernmentWalletPallet, 
        crate::mock::new_test_ext(), 
        crate::mock::Test
    );
}