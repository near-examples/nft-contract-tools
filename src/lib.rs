use crate::transfer_hook::TransferHook;
use near_sdk::{AccountId, NearToken, PanicOnDefault, near};
use near_sdk_contract_tools::{Owner, nft::*, owner::*};

#[derive(PanicOnDefault, Owner, NonFungibleToken)]
#[non_fungible_token(transfer_hook = "TransferHook")]
#[near(contract_state)]
pub struct MyNftContract {}

mod burn;
mod mint;
mod transfer_hook;

#[near]
impl MyNftContract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: ContractMetadata) -> Self {
        let mut contract = Self {};

        Owner::init(&mut contract, &owner_id);

        contract.set_contract_metadata(&metadata);

        contract.set_storage_balance_bounds(&StorageBalanceBounds {
            min: NearToken::from_yoctonear(7000000000000000000000),
            max: Some(NearToken::from_yoctonear(21000000000000000000000)),
        });

        contract
    }
}
