use frame::support::{
	dispatch::{result::Result,DispatchError,DispatchResult},
	traits::Get,
};

use sp_std::vec::Vec;

pub trait NonFungibleToken<AccountId>{
	type TokenId;
	type TokenAtributes;

	fn total() -> u64;
	fn burned() -> u64;
	fn total_of_account(account: &AccountId) -> u64;
	fn total_owned(account: &AccountId) -> Vec<(Self::TokenId, Self::TokenAtributes)>;
	fn owner_of(token_id: Self::TokenId) -> AccountId;
	fn mint(owner:AccountId, asset_attributes: Self::TokenAtributes) -> Result<Self::AssetId,DispatchError>;
	fn burn(asset_id: Self::AssetId) -> DispatchResult;
	fn transfer(from: AccountId, to: AccountId, asset_id: Self::AssetId) -> DispatchResult;
}
