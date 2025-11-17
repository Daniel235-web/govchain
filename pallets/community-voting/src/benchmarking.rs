#![cfg(feature = "runtime-benchmarks")]

use super::{Pallet as CommunityVotingPallet, *};
use frame::deps::frame_support::assert_ok;
use frame::{deps::frame_benchmarking::v2::*, prelude::*};

#[benchmarks]
mod benchmarks {
    use super::*;
    #[cfg(test)]
    use crate::pallet::Pallet as CommunityVotingPallet;
    use frame_system::RawOrigin;

    #[benchmark]
    fn start_voting() {
        let caller: T::AccountId = whitelisted_caller();
        
        // Setup: Prepare voting parameters
        let proposal_id = 1u64;
        let duration_blocks = 10u64;
        
        #[extrinsic_call]
        start_voting(
            RawOrigin::Signed(caller.clone()), 
            proposal_id, 
            duration_blocks
        );

        // Verification: Check that voting period was created and vote tally initialized
        assert!(VotingPeriods::<T>::get(proposal_id).is_some());
        
        // VoteCounts always return VoteTally (due to ValueQuery), so check if values are default
        let vote_tally = VoteCounts::<T>::get(proposal_id);
        assert_eq!(vote_tally.yes_votes, 0);
        assert_eq!(vote_tally.no_votes, 0);
        assert_eq!(vote_tally.abstain_votes, 0);
    }
    #[benchmark]
    fn cast_vote() {
        let creator: T::AccountId = whitelisted_caller();
        let voter: T::AccountId = whitelisted_caller(); // Different account
        
        // Setup: First start a voting period
        let proposal_id = 1u64;
        let duration_blocks = 10u64;
        
        assert_ok!(CommunityVotingPallet::<T>::start_voting(
            RawOrigin::Signed(creator.clone()).into(),
            proposal_id,
            duration_blocks
        ));

        // Cast a Yes vote (0)
        #[extrinsic_call]
        cast_vote(RawOrigin::Signed(voter.clone()), proposal_id, 0);

        // Verification: Check that vote was stored and tally updated
        assert!(ProposalVotes::<T>::contains_key(proposal_id, &voter));
        let vote_tally = VoteCounts::<T>::get(proposal_id);
        assert_eq!(vote_tally.yes_votes, 1);
        assert_eq!(vote_tally.no_votes, 0);
        assert_eq!(vote_tally.abstain_votes, 0);
    }
        #[benchmark]
    fn end_voting() {
        let creator: T::AccountId = whitelisted_caller();
        let ender: T::AccountId = whitelisted_caller(); // Different account
        
        // Setup: Start voting period with VERY SHORT duration that has already ended
        let proposal_id = 1u64;
        let duration_blocks = 0u64; // Voting ends immediately
        
        assert_ok!(CommunityVotingPallet::<T>::start_voting(
            RawOrigin::Signed(creator.clone()).into(),
            proposal_id,
            duration_blocks
        ));

        // Don't cast any votes - we just want to test the end_voting function
        // With no votes, result should be NoQuorum (2)

        #[extrinsic_call]
        end_voting(RawOrigin::Signed(ender.clone()), proposal_id);

        // Verification: Function should succeed 
        // The voting period was set to end immediately, so end_voting should work
    }
        #[benchmark]
    fn cast_vote_no() {
        let creator: T::AccountId = whitelisted_caller();
        let voter: T::AccountId = whitelisted_caller(); // Different account
        
        // Setup: First start a voting period
        let proposal_id = 2u64; // Different proposal ID
        let duration_blocks = 10u64;
        
        assert_ok!(CommunityVotingPallet::<T>::start_voting(
            RawOrigin::Signed(creator.clone()).into(),
            proposal_id,
            duration_blocks
        ));

        // Cast a No vote (1)
        #[extrinsic_call]
        cast_vote(RawOrigin::Signed(voter.clone()), proposal_id, 1);

        // Verification: Check that vote was stored and tally updated for No
        assert!(ProposalVotes::<T>::contains_key(proposal_id, &voter));
        let vote_tally = VoteCounts::<T>::get(proposal_id);
        assert_eq!(vote_tally.yes_votes, 0);
        assert_eq!(vote_tally.no_votes, 1);
        assert_eq!(vote_tally.abstain_votes, 0);
    }
    impl_benchmark_test_suite!(
        CommunityVotingPallet, 
        crate::mock::new_test_ext(), 
        crate::mock::Test
    );
}