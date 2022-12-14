use frame_support::dispatch::{result::Result, DispatchError, DispatchResult};
use sp_std::vec::Vec;
pub use sp_std::*;

pub trait NonFungibleToken<AccountId> {
	fn symbol() -> Vec<u8>;
	fn get_name() -> Vec<u8>;
	fn token_uri(token_id: Vec<u8>) -> Vec<u8>;
	fn total() -> u32;
	fn owner_of_token(token_id: Vec<u8>) -> AccountId;
	fn renting_price_of_token(token_id: Vec<u8>) -> u128;
	fn renting_time_of_token(token_id: Vec<u8>) -> u128;
	fn is_for_rent(token_id: Vec<u8>) -> bool;
	fn is_renter_of_token(who: AccountId, token_id: Vec<u8>) -> bool;
	fn is_borrowed_token(token_id: Vec<u8>) -> bool;

	fn mint(owner: AccountId) -> Result<Vec<u8>, DispatchError>;
	fn transfer(from: AccountId, to: AccountId, token_id: Vec<u8>) -> DispatchResult;
	fn set_token_uri(token_id: Vec<u8>, token_uri: Vec<u8>) -> DispatchResult;
	fn is_approve_for_all(account_approve: (AccountId, AccountId)) -> bool;
	fn approve(from: AccountId, to: AccountId, token_id: Vec<u8>) -> DispatchResult;
	fn set_approve_for_all(from: AccountId, to: AccountId) -> DispatchResult;
	fn set_token_renting_price(token_id: Vec<u8>, price: u128) -> DispatchResult;
	fn set_token_renting_time(token_id: Vec<u8>, time: u128) -> DispatchResult;
	fn set_token_for_rent(token_id: Vec<u8>) -> DispatchResult;
	fn set_token_cancel_for_rent(token_id: Vec<u8>) -> DispatchResult;
	fn set_token_giveback_block_number(token_id: Vec<u8>) -> DispatchResult;
}
