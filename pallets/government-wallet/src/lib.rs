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
        /// A new government wallet was created.
        GovernmentWalletCreated {
            /// The account ID of the new wallet.
            wallet_id: T::AccountId,
            /// The department or purpose of the wallet.
            department: Vec<u8>,
        },
        /// Funds were allocated to a government wallet.
        FundsAllocated {
            /// The wallet that received funds.
            wallet_id: T::AccountId,
            /// The amount allocated.
            amount: u128,
            /// The purpose of the allocation.
            purpose: Vec<u8>,
        },
    }
    /// Storage for government wallets and their metadata.
    #[pallet::storage]
    pub type GovernmentWallets<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        WalletDetails<T::AccountId>,
    >;

    /// Storage for wallet balances.
    #[pallet::storage]
    pub type WalletBalances<T: Config> = StorageMap<
        _,
        Twox64Concat,
        T::AccountId,
        u128,
        ValueQuery,
    >;

    #[pallet::error]
    pub enum Error<T> {
        /// Wallet already exists.
        WalletAlreadyExists,
        /// Wallet does not exist.
        WalletDoesNotExist,
        /// Invalid department name.
        InvalidDepartment,
    }

    // Wallet details structure
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct WalletDetails<AccountId> {
        /// The creator of the wallet.
        pub creator: AccountId,
        /// Department or purpose.
        pub department: BoundedVec<u8, ConstU32<100>>,
        /// When the wallet was created.
        pub created_at: u64,
    }
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new government wallet for a department.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `wallet_id`: The account ID for the new wallet.
        /// - `department`: The department name or purpose.
        ///
        /// Emits `GovernmentWalletCreated` event when successful.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_wallet())] 
        pub fn create_wallet(
            origin: OriginFor<T>,
            wallet_id: T::AccountId,
            department: Vec<u8>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            // Check if department name is valid
            if department.is_empty() {
                return Err(Error::<T>::InvalidDepartment.into());
            }
            if department.len() > 100 {
                return Err(Error::<T>::InvalidDepartment.into());
            }

            // Check if wallet already exists
            if GovernmentWallets::<T>::contains_key(&wallet_id) {
                return Err(Error::<T>::WalletAlreadyExists.into());
            }

            // Get current block number and convert to u64
            let current_block = frame_system::Pallet::<T>::block_number();
            let block_number: u64 = current_block.try_into().map_err(|_| Error::<T>::InvalidDepartment)?;

            // Convert department to bounded vec
            let bounded_department = BoundedVec::try_from(department.clone())
                .map_err(|_| Error::<T>::InvalidDepartment)?;

            // Create wallet details
            let wallet_details = WalletDetails {
                creator: creator.clone(),
                department: bounded_department,
                created_at: block_number,
            };

            // Store wallet details
            GovernmentWallets::<T>::insert(&wallet_id, wallet_details);

            // Initialize wallet balance to zero
            WalletBalances::<T>::insert(&wallet_id, 0u128);

            // Emit event
            Self::deposit_event(Event::<T>::GovernmentWalletCreated {
                wallet_id: wallet_id.clone(),
                department,
            });

            Ok(())
        }
            /// Allocate funds to a government wallet.
        ///
        /// The dispatch origin must be signed.
        ///
        /// - `wallet_id`: The wallet to receive funds.
        /// - `amount`: The amount to allocate.
        /// - `purpose`: The purpose of this allocation.
        ///
        /// Emits `FundsAllocated` event when successful.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::allocate_funds())] 
        pub fn allocate_funds(
            origin: OriginFor<T>,
            wallet_id: T::AccountId,
            amount: u128,
            purpose: Vec<u8>,
        ) -> DispatchResult {
            let _from = ensure_signed(origin)?;

            // Check if wallet exists
            if !GovernmentWallets::<T>::contains_key(&wallet_id) {
                return Err(Error::<T>::WalletDoesNotExist.into());
            }

            // Update wallet balance
            WalletBalances::<T>::mutate(&wallet_id, |balance| {
                *balance = balance.saturating_add(amount);
            });

            // Emit event
            Self::deposit_event(Event::<T>::FundsAllocated {
                wallet_id: wallet_id.clone(),
                amount,
                purpose,
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


