#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::{pallet_prelude::*};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn member_list_by_club)]
	pub type MemberListByClub<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// [club_account, member_account]
		MemberAddedToClub(T::AccountId, T::AccountId),
		/// [club_account, member_account]
		MemberRemovedFromClub(T::AccountId, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Member or club not available
		NotAvailable,
		/// Member under this club already exists
		Duplicate,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_member(origin: OriginFor<T>, club: T::AccountId, account: T::AccountId) -> DispatchResult {
			// Return early with error if a member under a certain club already exists
			if <MemberListByClub<T>>::contains_key(&club, &account) {
				return Err(Error::<T>::Duplicate.into());
			}
			ensure_root(origin)?;
			<MemberListByClub<T>>::insert(&club, &account, true);
			Self::deposit_event(Event::MemberAddedToClub(club, account));

			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_member(origin: OriginFor<T>, club: T::AccountId, account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			// Return early with error if a member under a certain club doesn't exist
			if !<MemberListByClub<T>>::contains_key(&club, &account) {
				return Err(Error::<T>::NotAvailable.into());
			}

			<MemberListByClub<T>>::remove(&club, &account);
			Self::deposit_event(Event::MemberAddedToClub(club, account));

			Ok(())
		}
	}
}
