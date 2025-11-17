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
        /// A new budget proposal was created.
        BudgetProposalCreated {
            /// The ID of the proposal.
            proposal_id: u64,
            /// The creator of the proposal.
            creator: T::AccountId,
            /// The amount requested.
            amount: u128,
            /// The purpose of the proposal.
            purpose: BoundedVec<u8, ConstU32<200>>,
        },
        /// A budget proposal was approved.
        BudgetProposalApproved {
            /// The ID of the proposal.
            proposal_id: u64,
            /// The approver account.
            approver: T::AccountId,
        },
        /// A budget proposal was rejected.
        BudgetProposalRejected {
            /// The ID of the proposal.
            proposal_id: u64,
            /// The rejecter account.
            rejecter: T::AccountId,
            /// The reason for rejection.
            reason: BoundedVec<u8, ConstU32<100>>,
        },
    }
    /// Storage for budget proposals.
    #[pallet::storage]
    pub type BudgetProposals<T: Config> = StorageMap<
        _,
        Twox64Concat,
        u64,
        ProposalDetails<T::AccountId>,
    >;

    /// Storage for the next proposal ID.
    #[pallet::storage]
    pub type NextProposalId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        /// Proposal does not exist.
        ProposalDoesNotExist,
        /// Proposal already exists.
        ProposalAlreadyExists,
        /// Invalid proposal purpose.
        InvalidPurpose,
        /// Invalid rejection reason.
        InvalidReason,
        /// Only the creator can update the proposal.
        NotProposalCreator,
        /// Proposal is not in pending state.
        ProposalNotPending,
    }

    /// Status of a budget proposal
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum ProposalStatus {
        /// Proposal is pending approval
        Pending,
        /// Proposal has been approved
        Approved,
        /// Proposal has been rejected
        Rejected,
    }

    /// Proposal details structure
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct ProposalDetails<AccountId> {
        /// The creator of the proposal
        pub creator: AccountId,
        /// The amount requested
        pub amount: u128,
        /// The purpose of the proposal
        pub purpose: BoundedVec<u8, ConstU32<200>>,
        /// Current status of the proposal
        pub status: ProposalStatus,
        /// When the proposal was created
        pub created_at: u64,
        /// When the proposal was last updated
        pub updated_at: u64,
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new budget proposal.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `amount`: The amount requested.
        /// - `purpose`: The purpose of the proposal.
        ///
        /// Emits `BudgetProposalCreated` event when successful.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_proposal())] 
        pub fn create_proposal(
            origin: OriginFor<T>,
            amount: u128,
            purpose: Vec<u8>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            // Check if purpose is valid
            ensure!(!purpose.is_empty(), Error::<T>::InvalidPurpose);
            ensure!(purpose.len() <= 200, Error::<T>::InvalidPurpose);

            // Convert purpose to bounded vec
            let bounded_purpose = BoundedVec::try_from(purpose.clone())
                .map_err(|_| Error::<T>::InvalidPurpose)?;

            // Get next proposal ID
            let proposal_id = NextProposalId::<T>::get();
            NextProposalId::<T>::put(proposal_id + 1);

            // Get current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            let block_number: u64 = current_block.try_into().map_err(|_| Error::<T>::InvalidPurpose)?;

            // Create proposal details
            let proposal_details = ProposalDetails {
                creator: creator.clone(),
                amount,
                purpose: bounded_purpose.clone(), // FIXED: Clone here
                status: ProposalStatus::Pending,
                created_at: block_number,
                updated_at: block_number,
            };

            // Store proposal
            BudgetProposals::<T>::insert(proposal_id, proposal_details);

            // Emit event
            Self::deposit_event(Event::<T>::BudgetProposalCreated {
                proposal_id,
                creator: creator.clone(),
                amount,
                purpose: bounded_purpose, // Original still available
            });

            Ok(())
        }

        /// Approve a budget proposal.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `proposal_id`: The ID of the proposal to approve.
        ///
        /// Emits `BudgetProposalApproved` event when successful.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::approve_proposal())] 
        pub fn approve_proposal(
            origin: OriginFor<T>,
            proposal_id: u64,
        ) -> DispatchResult {
            let approver = ensure_signed(origin)?;

            // Get proposal
            let mut proposal = BudgetProposals::<T>::get(proposal_id)
                .ok_or(Error::<T>::ProposalDoesNotExist)?;

            // Check if proposal is pending
            ensure!(
                proposal.status == ProposalStatus::Pending,
                Error::<T>::ProposalNotPending
            );

            // Update proposal status
            proposal.status = ProposalStatus::Approved;
            
            // Get current block number for update timestamp
            let current_block = frame_system::Pallet::<T>::block_number();
            let block_number: u64 = current_block.try_into().map_err(|_| Error::<T>::InvalidPurpose)?;
            proposal.updated_at = block_number;

            // Store updated proposal
            BudgetProposals::<T>::insert(proposal_id, proposal);

            // Emit event
            Self::deposit_event(Event::<T>::BudgetProposalApproved {
                proposal_id,
                approver: approver.clone(),
            });

            Ok(())
        }

        /// Reject a budget proposal.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `proposal_id`: The ID of the proposal to reject.
        /// - `reason`: The reason for rejection.
        ///
        /// Emits `BudgetProposalRejected` event when successful.
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::reject_proposal())] 
        pub fn reject_proposal(
            origin: OriginFor<T>,
            proposal_id: u64,
            reason: Vec<u8>,
        ) -> DispatchResult {
            let rejecter = ensure_signed(origin)?;

            // Check if reason is valid
            ensure!(!reason.is_empty(), Error::<T>::InvalidReason);
            ensure!(reason.len() <= 100, Error::<T>::InvalidReason);

            // Convert reason to bounded vec
            let bounded_reason = BoundedVec::try_from(reason.clone())
                .map_err(|_| Error::<T>::InvalidReason)?;

            // Get proposal
            let mut proposal = BudgetProposals::<T>::get(proposal_id)
                .ok_or(Error::<T>::ProposalDoesNotExist)?;

            // Check if proposal is pending
            ensure!(
                proposal.status == ProposalStatus::Pending,
                Error::<T>::ProposalNotPending
            );

            // Update proposal status
            proposal.status = ProposalStatus::Rejected;
            
            // Get current block number for update timestamp
            let current_block = frame_system::Pallet::<T>::block_number();
            let block_number: u64 = current_block.try_into().map_err(|_| Error::<T>::InvalidPurpose)?;
            proposal.updated_at = block_number;

            // Store updated proposal
            BudgetProposals::<T>::insert(proposal_id, proposal);

            // Emit event
            Self::deposit_event(Event::<T>::BudgetProposalRejected {
                proposal_id,
                rejecter: rejecter.clone(),
                reason: bounded_reason,
            });

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


