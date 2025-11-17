#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use scale_info::prelude::vec::Vec;


pub mod weights;
use crate::weights::WeightInfo;

#[frame::pallet]
pub mod pallet {
    use super::*;
    use frame::prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Configuration trait for the pallet.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type of the runtime.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An audit entry was created.
        AuditEntryCreated {
            /// The ID of the audit entry.
            entry_id: u64,
            /// The type of activity logged.
            activity_type: BoundedVec<u8, ConstU32<50>>,
            /// The account responsible for the activity.
            actor: T::AccountId,
            /// Details of the activity.
            details: BoundedVec<u8, ConstU32<500>>,
        },
    }
    /// Storage for audit log entries.
    #[pallet::storage]
    pub type AuditLog<T: Config> = StorageMap<
        _,
        Twox64Concat,
        u64, // entry_id
        AuditEntry<T::AccountId>,
    >;

    /// Storage for the next audit entry ID.
    #[pallet::storage]
    pub type NextEntryId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        /// Audit entry does not exist.
        EntryDoesNotExist,
        /// Invalid activity type.
        InvalidActivityType,
        /// Invalid details.
        InvalidDetails,
    }

    /// Audit entry structure
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct AuditEntry<AccountId> {
        /// The type of activity
        pub activity_type: BoundedVec<u8, ConstU32<50>>,
        /// The account that performed the activity
        pub actor: AccountId,
        /// Detailed description of the activity
        pub details: BoundedVec<u8, ConstU32<500>>,
        /// When the activity occurred
        pub timestamp: u64,
        /// Related proposal ID (if applicable)
        pub related_proposal_id: Option<u64>,
        /// Related wallet ID (if applicable)
        pub related_wallet_id: Option<AccountId>,
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new audit log entry.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `activity_type`: The type of activity being logged.
        /// - `details`: Detailed description of the activity.
        /// - `related_proposal_id`: Optional related proposal ID.
        /// - `related_wallet_id`: Optional related wallet ID.
        ///
        /// Emits `AuditEntryCreated` event when successful.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::log_activity())] 
        pub fn log_activity(
            origin: OriginFor<T>,
            activity_type: Vec<u8>,
            details: Vec<u8>,
            related_proposal_id: Option<u64>,
            related_wallet_id: Option<T::AccountId>,
        ) -> DispatchResult {
            let actor = ensure_signed(origin)?;

            // Validate activity type
            ensure!(!activity_type.is_empty(), Error::<T>::InvalidActivityType);
            ensure!(activity_type.len() <= 50, Error::<T>::InvalidActivityType);

            // Validate details
            ensure!(!details.is_empty(), Error::<T>::InvalidDetails);
            ensure!(details.len() <= 500, Error::<T>::InvalidDetails);

            // Convert to bounded vecs
            let bounded_activity_type = BoundedVec::try_from(activity_type.clone())
                .map_err(|_| Error::<T>::InvalidActivityType)?;
            let bounded_details = BoundedVec::try_from(details.clone())
                .map_err(|_| Error::<T>::InvalidDetails)?;

            // Get next entry ID
            let entry_id = NextEntryId::<T>::get();
            NextEntryId::<T>::put(entry_id + 1);

            // Get current block number for timestamp
            let current_block = frame_system::Pallet::<T>::block_number();
            let timestamp: u64 = current_block.try_into().map_err(|_| Error::<T>::InvalidActivityType)?;

            // Create audit entry
            let audit_entry = AuditEntry {
                activity_type: bounded_activity_type.clone(),
                actor: actor.clone(),
                details: bounded_details.clone(),
                timestamp,
                related_proposal_id,
                related_wallet_id,
            };

            // Store audit entry
            AuditLog::<T>::insert(entry_id, audit_entry);

            // Emit event
            Self::deposit_event(Event::<T>::AuditEntryCreated {
                entry_id,
                activity_type: bounded_activity_type,
                actor: actor.clone(),
                details: bounded_details,
            });

            Ok(())
        }

        /// Get an audit entry by ID.
        ///
        /// This is a read-only function that doesn't modify state.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::get_audit_entry())] 
        pub fn get_audit_entry(
            origin: OriginFor<T>,
            entry_id: u64,
        ) -> DispatchResult {
            let _requester = ensure_signed(origin)?;

            // Get audit entry - this will return an error if not found
            let _entry = AuditLog::<T>::get(entry_id)
                .ok_or(Error::<T>::EntryDoesNotExist)?;

            // In a real implementation, you would return the entry data
            // For now, we just verify it exists and return success

            Ok(())
        }
    }
}
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;



