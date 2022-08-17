use frame::support::{
	dispatch::{result::Result,DispatchError,DispatchResult},
	traits::Get,
};
pub use sp_std::*;
pub use frame_support::{pallet_prelude::Member, traits::Currency};

use sp_std::vec::Vec;

pub trait NonFungibleToken<AccountId>{
	type TokenId: Default+ Parameter+ Copy ;
	type Currency: Currency<AccountId>;

	fn symbol() -> Vec<u8>;
	fn name() -> Vec<u8>;
	fn token_uri(token_id: Self::TokenId) -> Vec<u8>;
	fn total() -> u64;

	fn total_of_account(account: &AccountId) -> u64;
	fn total_owned(account: &AccountId) -> Vec<(Self::TokenId, Self::TokenAtributes)>;
	fn owner_of(token_id: Self::TokenId) -> AccountId;
	fn mint(owner:AccountId, asset_attributes: Self::TokenAtributes) -> Result<Self::AssetId,DispatchError>;

	fn burn(asset_id: Self::AssetId) -> DispatchResult;
	fn transfer(from: AccountId, to: AccountId, asset_id: Self::AssetId) -> DispatchResult;

	fn get_approve(token_id: Self::TokenId) -> Option<AccountId>;
	fn is_approve_for_all(token_id: Self::TokenId) -> bool;

	fn approve(from: &AccountId, to: &AccountId,token_id: Self::TokenId) -> DispatchResult;
	fn set_approve_for_all(from: &AccountId, to: &AccountId, token_id: Self::TokenId) -> DispatchResult;

}
