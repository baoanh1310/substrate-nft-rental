#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
pub mod nft;
use frame_support::{dispatch::{result::Result, DispatchError, DispatchResult}, ensure, traits::Get};
pub use frame_support::{pallet_prelude::{Member, StorageMap,StorageValue}};
pub use frame_support::traits::Currency;
pub use nft::NonFungibleToken;

pub use frame_system::{ensure_signed, ensure_root};
pub use frame_system::EnsureRoot;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub use sp_std::{vec::Vec, convert::Into};

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::Currency;
	use frame_system::pallet_prelude::*;
	pub use super::*;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TokenId: Parameter+ Member+ Default + Copy +Into<u64>;
		type Currency: Currency<Self::AccountId>;
		// type Administrator : EnsureOrigin<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn name)]
	// name of the nft
	pub (super) type Name<T> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn symbol)]
	// symbol of the nft
	pub (super) type Symbol<T> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_uri)]
	// uri of the nft
	pub (super) type TokenUri<T: Config>  = StorageMap<_, Blake2_128Concat, T::TokenId, Vec<u8>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_tokens)]
	// total count of the token
	pub (super) type TotalTokens<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_owner)]
	// Mapping Token Id => Account Id: to check who is the owner of the token
	pub (super) type TokenOwner<T:Config> = StorageMap<_,Blake2_128Concat,T::TokenId,T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner_token)]
	// To check all the token that the account owns
	pub (super) type OwnerToken<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::TokenId>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn is_approve_for_all)]
	// To check all the token that the account owns
	pub (super) type Approval<T:Config> = StorageMap<_, Blake2_128Concat, (T::AccountId,T::AccountId),bool, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_approval)]
	pub (super) type TokenApproval<T:Config> = StorageMap<_, Blake2_128Concat, T::TokenId, Vec<T::AccountId>, OptionQuery>;

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
		// #[pallet::weight(46_367_000 + T::DbWeight::get().reads_writes(6, 4))]
		pub fn mint(origin: OriginFor<T>, to: T::AccountId, token_id:T::TokenId) -> DispatchResult {
			ensure!(ensure_signed(origin)==)
		}

	}
}

// helper functions
impl <T:Config>  Pallet<T>{
	fn do_approve(from: &T::AccountId, to: &T::AccountId, token_id: T::TokenId) -> DispatchResult{
		let owner = TokenOwner::<T>::get(token_id).unwrap();
		ensure!(*from==owner || Self::is_approve_for_all((owner.clone(), from.clone())).unwrap(), "Not Owner nor approved");
		TokenApproval::<T>::mutate(token_id, |list_account|{
			if let Some(l) = list_account {
				l.push(to.clone());
			}
		} );
		Ok(())
	}

	fn do_approve_for_all(from: &T::AccountId, to: &T::AccountId, approved: bool) -> DispatchResult{
		let account = (from,to);
		Approval::<T>::insert(account,true);
		Ok(())
	}
}

impl<T: Config> NonFungibleToken<T::AccountId> for Pallet<T>{
	type TokenId = T::TokenId;
	type Currency = T::Currency;
	// type Administrator = T::ForceOrigin;
	// fn administrator() -> T::AccountId{
	// 	Self::administrator()
	// }
	fn symbol() -> Vec<u8> {
		Self::symbol()
	}

	fn name() -> Vec<u8>{
		Self::name()
	}

	fn token_uri(token_id: Self::TokenId) -> Vec<u8> {
		Self::token_uri(token_id).unwrap()
	}

	fn total() -> u32 {
		Self::total_tokens()
	}

	fn total_of_account(account: &T::AccountId) -> u64 {
		Self::total_of_account(account)
	}

	fn total_owned(account: &T::AccountId) -> Vec<(Self::TokenId, Vec<u8>)> {
		Self::total_owned(account)
	}

	fn owner_of(token_id: Self::TokenId) -> T::AccountId {
		Self::owner_of(token_id)
	}

	fn mint(account: T::AccountId, token_id: Self::TokenId) -> Result<Self::TokenId, DispatchError> {
		TotalTokens::<T>::mutate(|value| *value+=1);
		TokenOwner::<T>::mutate(token_id, |owner| {
			if let Some(t) = owner {
				*t = account.clone();
			}
		});
		OwnerToken::<T>::mutate(account,|list_token| {
			if let Some(list) = list_token {
				list.push(token_id)
			}
		});
		Ok(token_id)
	}

	fn transfer(from: T::AccountId, to: T::AccountId, token_id: Self::TokenId) -> DispatchResult {
		ensure!(from == Self::owner_of(token_id), Error::<T>::NotOwner);
		TokenOwner::<T>::mutate(token_id, |owner| {
			if let Some(o) = owner {
				*o = to.clone()
			}
		});
		TokenOwner::<T>::mutate(token_id, |owner| *owner = Some(to.clone()));
		let list_token = OwnerToken::<T>::get(&from);
		let mut index = 0;
		for token in list_token.unwrap().iter() {
			if *token == token_id {
				break;
			}
			index += 1;
		}
		OwnerToken::<T>::mutate(from,|list_token| {
			if let Some(list) = list_token {
				list.remove(index);
			}
		});
		Ok(())
	}

	fn is_approve_for_all(account_approve:(T::AccountId, T::AccountId)) -> bool {
		Self::is_approve_for_all(account_approve).unwrap()
	}

	fn approve(from: &T::AccountId, to: &T::AccountId, token_id: Self::TokenId) -> DispatchResult {
		Self::do_approve(from, to, token_id)
	}

	fn set_approve_for_all(from: &T::AccountId, to: &T::AccountId,approved:bool) -> DispatchResult {
		ensure!(from!=to, Error::<T>::Invalid);
		Self::do_approve_for_all(from,to,approved);
		Ok(())
	}
}
