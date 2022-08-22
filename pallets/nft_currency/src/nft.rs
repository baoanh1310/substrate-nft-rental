use frame_support::{dispatch::{result::Result, DispatchError, DispatchResult,EncodeLike,MaxEncodedLen}, Parameter, traits::Get};
pub use sp_std::*;
pub use frame_support::{pallet_prelude::Member, traits::Currency};
use frame_support::traits::EnsureOrigin;
use sp_std::vec::Vec;

pub trait NonFungibleToken<AccountId>{
	// type TokenId: Parameter+ Member +MaxEncodedLen+EncodeLike + Default+ Copy+Into<u64> ;
	type Currency: Currency<AccountId>;
	// type Administrator : EnsureOrigin<Self::AccountId>;
	//
	// fn administrator() -> AccountId;
	fn symbol() -> Vec<u8>;
	fn name() -> Vec<u8>;
	fn token_uri(token_id: Vec<u8>) -> Vec<u8>;
	fn total() -> u32;

	fn total_of_account(account: &AccountId) -> u64;
	fn owner_of(token_id: Vec<u8>) -> AccountId;
	fn mint(owner:AccountId) -> Result<Vec<u8>,DispatchError>;
	fn transfer(from: AccountId, to: AccountId, token_id: Vec<u8>) -> DispatchResult;

	fn is_approve_for_all(account_approve:(AccountId,AccountId)) -> bool;

	fn approve(from: &AccountId, to: &AccountId,token_id: Vec<u8>) -> DispatchResult;
	fn set_approve_for_all(from: &AccountId, to: &AccountId, approved:bool) -> DispatchResult;

}
