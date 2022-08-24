#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
mod nft;
use frame_support::{dispatch::{result::Result, DispatchError, DispatchResult}, ensure, traits::{Get,Randomness}};
use frame_support::{pallet_prelude::{StorageMap,StorageValue}};
use frame_support::traits::Currency;
use frame_support::traits::EnsureOrigin;
use nft::NonFungibleToken;
use frame_system::{ensure_signed};

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
	use frame_support::traits::{Currency, Randomness};
	use frame_system::pallet_prelude::*;
	pub use super::*;
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		// type Administrator : EnsureOrigin<Self::Origin>;
		type Randomness : Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn nft_name)]
	// name of the nft
	pub (super) type Name<T> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn symbol)]
	// symbol of the nft
	pub (super) type Symbol<T> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_uri)]
	// uri of the nft
	pub (super) type TokenUri<T: Config>  = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<u8>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_tokens)]
	// total count of the token
	pub (super) type TotalTokens<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_owner)]
	// Mapping Token Id => Account Id: to check who is the owner of the token
	pub (super) type TokenOwner<T:Config> = StorageMap<_,Blake2_128Concat,Vec<u8>,T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner_token)]
	// To check all the token that the account owns
	pub (super) type OwnerToken<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn is_approve_for_all)]
	// To check all the token that the account owns
	pub (super) type Approval<T:Config> = StorageMap<_, Blake2_128Concat, (T::AccountId,T::AccountId),bool, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn token_approval)]
	pub (super) type TokenApproval<T:Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Vec<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Mint(T::AccountId, Vec<u8>),
		Transfer(T::AccountId, T::AccountId, Vec<u8>),
		Approve(T::AccountId, T::AccountId, Vec<u8>),
		ApproveForAll(T::AccountId, T::AccountId),
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
		NoneExist,
		NotOwnerNorOperator,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000_000)]
		pub fn mint_to(origin: OriginFor<T>, to: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let token_id = <Self as NonFungibleToken<_>>::mint(to.clone())?;
			Self::deposit_event(Event::Mint(to,token_id));
			Ok(())
		}

		#[pallet::weight(1_000_000)]
		pub fn transfer_token(origin: OriginFor<T>, to: T::AccountId, token_id:Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who == Self::token_owner(token_id.clone()).unwrap() ,Error::<T>::NotOwner);
			<Self as NonFungibleToken<_>>::transfer(who.clone(), to.clone(), token_id.clone());
			Self::deposit_event(Event::Transfer(who,to,token_id));
			Ok(())
		}

		#[pallet::weight(1_000_000)]
		pub fn safe_transfer(origin: OriginFor<T>,from: T::AccountId, to: T::AccountId, token_id:Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let account = (from.clone(),who.clone());
			ensure!(who == Self::token_owner(token_id.clone()).unwrap() || Self::is_approve_for_all(account).unwrap(),Error::<T>::NotOwnerNorOperator);

			<Self as NonFungibleToken<_>>::transfer(from.clone(),to.clone(),token_id.clone())?;
			Self::deposit_event(Event::Transfer(from,to,token_id));
			Ok(())
		}

		#[pallet::weight(1_000_000)]
		pub fn approve(origin: OriginFor<T>, to: T::AccountId, token_id:Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let account = (who.clone(),to.clone());
			ensure!(who == Self::token_owner(token_id.clone()).unwrap(),Error::<T>::NotOwner);
			<Self as NonFungibleToken<_>>::approve(who.clone(), to.clone(), token_id.clone())?;
			Self::deposit_event(Event::Approve(who,to,token_id));
			Ok(())
		}

		#[pallet::weight(1_000_000)]
		pub fn approve_for_all(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			<Self as NonFungibleToken<_>>::set_approve_for_all(who.clone(), account.clone())?;
			Self::deposit_event(Event::ApproveForAll(who,account));
			Ok(())
		}

		#[pallet::weight(1_000_000)]
		pub fn set_token_uri(origin: OriginFor<T>, token_id: Vec<u8>,token_uri:Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who == Self::token_owner(token_id.clone()).unwrap(),Error::<T>::NotOwner);
			<Self as NonFungibleToken<_>>::set_token_uri(token_id,token_uri)?;
			Ok(())
		}
	}
}

// helper functions
impl <T:Config>  Pallet<T>{
	fn gen_token_id() -> Vec<u8> {
		let nonce = TotalTokens::<T>::get();
		let n = nonce.encode();
		let (rand, _) = T::Randomness::random(&n);
		rand.encode()
	}

}


impl<T: Config> NonFungibleToken<T::AccountId> for Pallet<T>{
	type Currency = T::Currency;

	fn symbol() -> Vec<u8> {
		Symbol::<T>::get()
	}

	fn get_name() -> Vec<u8>{
		Name::<T>::get()
	}

	fn token_uri(token_id: Vec<u8>) -> Vec<u8> {
		TokenUri::<T>::get(token_id).unwrap()
	}

	fn total() -> u32 {
		TotalTokens::<T>::get()
	}

	fn owner_of_token(token_id: Vec<u8>) -> T::AccountId {
		Self::token_owner(token_id).unwrap()
	}

	fn mint(owner: T::AccountId) -> Result<Vec<u8>, DispatchError>  {
		let token_id = Self::gen_token_id();
		TotalTokens::<T>::mutate(|value| *value+=1);
		TokenOwner::<T>::mutate(token_id.clone(), |account| {
			*account = Some(owner.clone());

		});
		OwnerToken::<T>::mutate(owner,|list_token| {
			list_token.push(token_id.clone());
		});
		Ok(token_id)
	}

	fn transfer(from: T::AccountId, to: T::AccountId, token_id: Vec<u8>) -> DispatchResult {
		ensure!(from == Self::owner_of_token(token_id.clone()), Error::<T>::NotOwner);
		TokenOwner::<T>::mutate(token_id.clone(), |owner| *owner = Some(to.clone()));
		OwnerToken::<T>::mutate(to,|list_token| {
			list_token.push(token_id.clone());
		});
		OwnerToken::<T>::mutate(from,|list_token| {
			if let Some(ind) = list_token.iter().position(|id| *id == token_id) {
				list_token.swap_remove(ind);
				return Ok(())
			}
			Err(())
		});
		Ok(())
	}

	fn is_approve_for_all(account_approve:(T::AccountId, T::AccountId)) -> bool {
		Approval::<T>::get(account_approve).unwrap()
	}

	fn approve(from: T::AccountId, to: T::AccountId, token_id: Vec<u8>) -> DispatchResult {
		let owner = TokenOwner::<T>::get(token_id.clone()).unwrap();
		ensure!(from==owner, "Not Owner nor approved");
		TokenApproval::<T>::mutate(token_id.clone(), |list_account|{
			list_account.push(to);
		} );
		Ok(())
	}

	fn set_approve_for_all(from: T::AccountId, to: T::AccountId) -> DispatchResult {
		let account = (from,to);
		Approval::<T>::mutate(account, |approved|{
			*approved = Some(true);
		} );
		Ok(())
	}

	fn set_token_uri(token_id: Vec<u8>, token_uri:Vec<u8>) -> DispatchResult{
		TokenUri::<T>::mutate(token_id,|uri| *uri = Some(token_uri));
		Ok(())
	}
}
