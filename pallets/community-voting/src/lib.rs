#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

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
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A vote was cast on a proposal.
        VoteCast {
            /// The proposal ID that was voted on.
            proposal_id: u64,
            /// The account that cast the vote.
            voter: T::AccountId,
            /// The vote choice (0=Yes, 1=No, 2=Abstain).
            vote: u8,
        },
        /// Voting period for a proposal has ended.
        VotingPeriodEnded {
            /// The proposal ID.
            proposal_id: u64,
            /// The final result (0=Passed, 1=Failed, 2=NoQuorum).
            result: u8,
        },
    }
    /// Storage for votes on each proposal.
    #[pallet::storage]
    pub type ProposalVotes<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        u64, // proposal_id
        Twox64Concat,
        T::AccountId, // voter
        VoteChoice,
    >;

    /// Storage for vote counts per proposal.
    #[pallet::storage]
    pub type VoteCounts<T: Config> = StorageMap<
        _,
        Twox64Concat,
        u64, // proposal_id
        VoteTally,
        ValueQuery,
    >;

    /// Storage for voting periods.
    #[pallet::storage]
    pub type VotingPeriods<T: Config> = StorageMap<
        _,
        Twox64Concat,
        u64, // proposal_id
        VotingPeriod,
    >; // REMOVED ValueQuery

    #[pallet::error]
    pub enum Error<T> {
        /// Voting period has ended.
        VotingPeriodEnded,
        /// Voting period has not started.
        VotingPeriodNotStarted,
        /// Account has already voted on this proposal.
        AlreadyVoted,
        /// Proposal does not exist.
        ProposalDoesNotExist,
    }

    /// Vote choice options
    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum VoteChoice {
        /// Vote in favor
        Yes,
        /// Vote against
        No,
        /// Abstain from voting
        Abstain,
    }

    /// Vote result
    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum VoteResult {
        /// Proposal passed
        Passed,
        /// Proposal failed
        Failed,
        /// No quorum reached
        NoQuorum,
    }

    /// Vote tally structure
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
    pub struct VoteTally {
        /// Number of yes votes
        pub yes_votes: u32,
        /// Number of no votes
        pub no_votes: u32,
        /// Number of abstain votes
        pub abstain_votes: u32,
    }

    /// Voting period details
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct VotingPeriod {
        /// When voting started
        pub start_block: u64,
        /// When voting ends
        pub end_block: u64,
    }
        #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Start a voting period for a proposal.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `proposal_id`: The ID of the proposal to start voting on.
        /// - `duration_blocks`: How many blocks the voting period should last.
        ///
        /// Emits `VotingPeriodEnded` event when the period ends.
        #[pallet::call_index(0)]
        #[pallet::weight(0)]
        pub fn start_voting(
            origin: OriginFor<T>,
            proposal_id: u64,
            duration_blocks: u64,
        ) -> DispatchResult {
            let _starter = ensure_signed(origin)?;

            // Get current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            let start_block: u64 = current_block.try_into().map_err(|_| Error::<T>::ProposalDoesNotExist)?;

            // Calculate end block
            let end_block = start_block + duration_blocks;

            // Create voting period
            let voting_period = VotingPeriod {
                start_block,
                end_block,
            };

            // Store voting period
            VotingPeriods::<T>::insert(proposal_id, voting_period);

            // Initialize vote tally
            let initial_tally = VoteTally::default();
            VoteCounts::<T>::insert(proposal_id, initial_tally);

            Ok(())
        }

        /// Cast a vote on a proposal.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `proposal_id`: The ID of the proposal to vote on.
        /// - `vote`: The vote choice (0=Yes, 1=No, 2=Abstain).
        ///
        /// Emits `VoteCast` event when successful.
        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        pub fn cast_vote(
            origin: OriginFor<T>,
            proposal_id: u64,
            vote: u8,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            // Validate vote choice
            let vote_choice = match vote {
                0 => VoteChoice::Yes,
                1 => VoteChoice::No,
                2 => VoteChoice::Abstain,
                _ => return Err(Error::<T>::ProposalDoesNotExist.into()),
            };

            // Get voting period
            let voting_period = VotingPeriods::<T>::get(proposal_id)
                .ok_or(Error::<T>::ProposalDoesNotExist)?;

            // Get current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            let current_block_u64: u64 = current_block.try_into().map_err(|_| Error::<T>::ProposalDoesNotExist)?;

            // Check if voting period is active
            ensure!(
                current_block_u64 >= voting_period.start_block && 
                current_block_u64 <= voting_period.end_block,
                Error::<T>::VotingPeriodEnded
            );

            // Check if voter has already voted
            ensure!(
                !ProposalVotes::<T>::contains_key(proposal_id, &voter),
                Error::<T>::AlreadyVoted
            );

            // Store the vote
            ProposalVotes::<T>::insert(proposal_id, &voter, vote_choice);

            // Update vote tally
            VoteCounts::<T>::mutate(proposal_id, |tally| {
                match vote_choice {
                    VoteChoice::Yes => tally.yes_votes += 1,
                    VoteChoice::No => tally.no_votes += 1,
                    VoteChoice::Abstain => tally.abstain_votes += 1,
                }
            });

            // Emit event with simple u8 instead of enum
            Self::deposit_event(Event::<T>::VoteCast {
                proposal_id,
                voter: voter.clone(),
                vote,
            });

            Ok(())
        }

        /// End voting period and calculate result.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `proposal_id`: The ID of the proposal to end voting for.
        ///
        /// Emits `VotingPeriodEnded` event with the result.
        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        pub fn end_voting(
            origin: OriginFor<T>,
            proposal_id: u64,
        ) -> DispatchResult {
            let _ender = ensure_signed(origin)?;

            // Get voting period
            let voting_period = VotingPeriods::<T>::get(proposal_id)
                .ok_or(Error::<T>::ProposalDoesNotExist)?;

            // Get current block number
            let current_block = frame_system::Pallet::<T>::block_number();
            let current_block_u64: u64 = current_block.try_into().map_err(|_| Error::<T>::ProposalDoesNotExist)?;

            // Check if voting period has ended
            ensure!(
                current_block_u64 > voting_period.end_block,
                Error::<T>::VotingPeriodNotStarted
            );

            // Get vote tally
            let vote_tally = VoteCounts::<T>::get(proposal_id);

            // Calculate result (simple majority)
            let total_votes = vote_tally.yes_votes + vote_tally.no_votes;
            let result = if total_votes == 0 {
                2 // NoQuorum
            } else if vote_tally.yes_votes > vote_tally.no_votes {
                0 // Passed
            } else {
                1 // Failed
            };

            // Emit event with simple u8 instead of enum
            Self::deposit_event(Event::<T>::VotingPeriodEnded {
                proposal_id,
                result,
            });

            Ok(())
        }
    }
}