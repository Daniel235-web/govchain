#![cfg(feature = "runtime-benchmarks")]

use super::{Pallet as BudgetProposalPallet, *};
use frame::deps::frame_support::assert_ok;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as BudgetProposalPallet;
    use frame_system::RawOrigin;

    #[benchmark]
    fn create_proposal() {
        let caller: T::AccountId = whitelisted_caller();
        
        // Setup: Prepare valid proposal data
        let amount = 1000u128;
        let purpose = b"Community development project".to_vec();
        
        #[extrinsic_call]
        create_proposal(
            RawOrigin::Signed(caller.clone()), 
            amount, 
            purpose
        );

        // Verification: Check that proposal was created
        assert_eq!(NextProposalId::<T>::get(), 1);
        assert!(BudgetProposals::<T>::get(0).is_some());
    }
    #[benchmark]
    fn approve_proposal() {
        let creator: T::AccountId = whitelisted_caller();
        let approver: T::AccountId = whitelisted_caller(); // Different account
        
        // Setup: First create a proposal to approve
        assert_ok!(BudgetProposalPallet::<T>::create_proposal(
            RawOrigin::Signed(creator.clone()).into(),
            1000u128,
            b"Infrastructure upgrade".to_vec()
        ));

        #[extrinsic_call]
        approve_proposal(RawOrigin::Signed(approver.clone()), 0);

        // Verification: Check that proposal status changed to Approved
        let proposal = BudgetProposals::<T>::get(0).expect("Proposal should exist");
        assert_eq!(proposal.status, ProposalStatus::Approved);
    }
    #[benchmark]
    fn reject_proposal() {
        let creator: T::AccountId = whitelisted_caller();
        let rejecter: T::AccountId = whitelisted_caller(); // Different account
        
        // Setup: First create a proposal to reject
        assert_ok!(BudgetProposalPallet::<T>::create_proposal(
            RawOrigin::Signed(creator.clone()).into(),
            2000u128,
            b"Marketing campaign".to_vec()
        ));

        // Prepare maximum size reason (100 bytes)
        let reason = b"X".repeat(100);

        #[extrinsic_call]
        reject_proposal(RawOrigin::Signed(rejecter.clone()), 0, reason);

        // Verification: Check that proposal status changed to Rejected
        let proposal = BudgetProposals::<T>::get(0).expect("Proposal should exist");
        assert_eq!(proposal.status, ProposalStatus::Rejected);
    }
    #[benchmark]
    fn create_proposal_with_max_sizes() {
        let caller: T::AccountId = whitelisted_caller();
        
        // Setup: Prepare maximum size data
        let amount = u128::MAX; // Maximum possible amount
        let purpose = b"A".repeat(200); // Maximum allowed purpose size

        #[extrinsic_call]
        create_proposal(
            RawOrigin::Signed(caller.clone()), 
            amount, 
            purpose
        );

        // Verification: Check that proposal was created with max data
        assert_eq!(NextProposalId::<T>::get(), 1);
        let proposal = BudgetProposals::<T>::get(0).expect("Proposal should exist");
        assert_eq!(proposal.amount, u128::MAX);
        assert_eq!(proposal.purpose.len(), 200);
        assert_eq!(proposal.status, ProposalStatus::Pending);
    }
    impl_benchmark_test_suite!(
        BudgetProposalPallet, 
        crate::mock::new_test_ext(), 
        crate::mock::Test
    );
}