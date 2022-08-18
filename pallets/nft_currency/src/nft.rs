use frame_support::{
	dispatch::{result::Result,DispatchError,DispatchResult},
	traits::Get,
};
pub use sp_std::*;
pub use frame_support::{pallet_prelude::Member, traits::Currency};
use frame_support::traits::EnsureOrigin;
use sp_std::vec::Vec;

pub trait NonFungibleToken<AccountId>{
	type TokenId: Member  + Default+ Copy+Into<u64> ;
	type Currency: Currency<AccountId>;
	// type Administrator : EnsureOrigin<Self::AccountId>;
	//
	// fn administrator() -> AccountId;
	fn symbol() -> Vec<u8>;
	fn name() -> Vec<u8>;
	fn token_uri(token_id: Self::TokenId) -> Vec<u8>;
	fn total() -> Self::TokenId;

	fn total_of_account(account: &AccountId) -> u64;
	fn total_owned(account: &AccountId) -> Vec<(Self::TokenId, Vec<u8>)>;
	fn owner_of(token_id: Self::TokenId) -> AccountId;
	fn mint(owner:AccountId, token_id: Self::TokenId) -> Result<Self::TokenId,DispatchError>;
	fn transfer(from: AccountId, to: AccountId, token_id: Self::TokenId) -> DispatchResult;

	fn is_approve_for_all(account_approve:(AccountId,AccountId)) -> bool;

	fn approve(from: &AccountId, to: &AccountId,token_id: Self::TokenId) -> DispatchResult;
	fn set_approve_for_all(from: &AccountId, to: &AccountId, token_id: Self::TokenId) -> DispatchResult;

}
