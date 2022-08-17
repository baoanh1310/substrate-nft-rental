#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>

pub mod nft;
pub use pallet::*;
use frame_support::{
	dispatch::{result::Result,DispatchError,DispatchResult},
	traits::Get,
};
pub use sp_std::*;
pub use frame_support::{pallet_prelude::{Member, StorageMap,StorageValue}};
pub use frame_support::traits::Currency;
use nft::{NonFungibleToken};
use sp_std::vec::Vec;
pub use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub use sp_std::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Currency;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TokenId: Default + Copy;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	// name of the nft
	pub type Name<T> = StorageValue<_, Vec<u8>, ValueQuery>;
	// symbol of the nft
	pub type Symbol<T> = StorageValue<_, Vec<u8>, ValueQuery>;
	// uri of the nft
	pub type TokenUri<T: Config>  = StorageMap<_, Blake2_128Concat, u64, Vec<u8>, ValueQuery>;
	// total count of the token
	pub type TotalTokens<T> = StorageValue<_, u32, OptionQuery>;

	// Mapping Token Id => Account Id: to check who is the owner of the token
	pub type TokenOwner<T: Config> = StorageMap<_,Blake2_128Concat,u64,AccountId, OptionQuery>;
	// To check all the token that the account owns
	pub type OwnerToken<T:Config> = StorageMap<_, Blake2_128Concat, AccountId, Vec<u64>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Mint(T::AccountId, u64),
		Burn(u64),
		Transfer(T::AccountId, T::AccountId, u64),
		Approve(T::AccountId, T::AccountId, u64),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		Invalid,
		NotOwner,
		NoneExist
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}

impl <T:Config>  Pallet<T>{

}

impl<T:Config> NonFungibleToken<T::AccountId> for Pallet<T>{
	type TokenId = T::TokenId;
	type Currency = T::Currency;

	fn symbol() -> Vec<u8> {
		Self::symbol()
	}

	fn name() -> Vec<u8>{
		Self::name()
	}

	fn token_uri(token_id: Self::TokenId) -> Vec<u8> {
		Self::token_uri(token_id)
	}

	fn total() -> Self::TokenId {
		Self::total()
	}

	fn total_of_account(account: &T::AccountId) -> u64 {
		Self::total_of_account(account)
	}

	fn total_owned(account: &T::AccountId) -> Vec<(Self::TokenId, Vec<u8>)> {
		Self::total_owned(account)
	}

	fn owner_of(token_id: Self::TokenId) -> T::AccountId {
		Self::owner_of(token_id);
	}

	fn mint(owner: T::Account, token_id: Self::TokenId) -> Result<Self::TokenId, DispatchError> {
		todo!()
	}

	fn burn(asset_id: Self::TokenId) -> DispatchResult {
		todo!()
	}

	fn transfer(from: T::Account, to: T::Account, asset_id: Self::TokenId) -> DispatchResult {
		todo!()
	}

	fn get_approve(token_id: Self::TokenId) -> Option<T::Account> {
		todo!()
	}

	fn is_approve_for_all(token_id: Self::TokenId) -> bool {
		todo!()
	}

	fn approve(from: &T::Account, to: &T::Account, token_id: Self::TokenId) -> DispatchResult {
		todo!()
	}

	fn set_approve_for_all(from: &T::Account, to: &T::Account, token_id: Self::TokenId) -> DispatchResult {
		todo!()
	}
}
