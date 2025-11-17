#![cfg(feature = "runtime-benchmarks")]

use super::{Pallet as AuditLogPallet, *};
use frame::deps::frame_support::assert_ok;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as AuditLogPallet;
    use frame_system::RawOrigin;

    #[benchmark]
    fn log_activity() {
        let caller: T::AccountId = whitelisted_caller();
        
        // Setup: Prepare valid activity data
        let activity_type = b"LOGIN".to_vec();
        let details = b"User logged into system".to_vec();
        
        #[extrinsic_call]
        log_activity(
            RawOrigin::Signed(caller.clone()), 
            activity_type, 
            details, 
            None, 
            None
        );

        // Verification: Check that entry was created
        assert_eq!(NextEntryId::<T>::get(), 1);
        assert!(AuditLog::<T>::get(0).is_some());
    }
    #[benchmark]
    fn get_audit_entry() {
        let caller: T::AccountId = whitelisted_caller();
        
        // Setup: First create an audit entry to retrieve
        assert_ok!(AuditLogPallet::<T>::log_activity(
            RawOrigin::Signed(caller.clone()).into(),
            b"LOGIN".to_vec(),
            b"User logged into system".to_vec(),
            None,
            None
        ));

        #[extrinsic_call]
        get_audit_entry(RawOrigin::Signed(caller.clone()), 0);

        // Verification: Entry exists (the function would return error if not)
        // No need for additional checks since the extrinsic_call already verifies success
    }
    #[benchmark]
    fn log_activity_with_max_sizes_and_related_ids() {
        let caller: T::AccountId = whitelisted_caller();
        
        // Setup: Prepare maximum size data and related IDs
        let activity_type = b"A".repeat(50); // Max allowed size
        let details = b"X".repeat(500); // Max allowed size
        let related_proposal_id = Some(100u64);
        let related_wallet_id = Some(whitelisted_caller()); // Use new whitelisted caller directly

        // Clone the values before passing to extrinsic_call
        let activity_type_clone = activity_type.clone();
        let details_clone = details.clone();
        let related_wallet_id_clone = related_wallet_id.clone();

        #[extrinsic_call]
        log_activity(
            RawOrigin::Signed(caller.clone()), 
            activity_type, 
            details, 
            related_proposal_id, 
            related_wallet_id
        );

        // Verification: Check that entry was created with max data
        assert_eq!(NextEntryId::<T>::get(), 1);
        let entry = AuditLog::<T>::get(0).expect("Entry should exist");
        assert_eq!(entry.activity_type.len(), 50);
        assert_eq!(entry.details.len(), 500);
        assert_eq!(entry.related_proposal_id, related_proposal_id);
        assert_eq!(entry.related_wallet_id, related_wallet_id_clone);
    }
    impl_benchmark_test_suite!(
        AuditLogPallet, 
        crate::mock::new_test_ext(), 
        crate::mock::Test
    );
}