use crate::{MyNftContract, MyNftContractExt};
use near_sdk::{AccountId, env, log, near};
use near_sdk_contract_tools::{
    ft::Nep145,
    nft::{Nep177Controller, TokenId, TokenMetadata},
};

#[near]
impl MyNftContract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        owner_id: Option<AccountId>,
    ) {
        // Check account's storage balance and deposit if necessary
        let storage_balance_bounds = self.storage_balance_bounds();
        log!("Storage balance bounds: {:?}", storage_balance_bounds);

        let storage_balance = self
            .storage_balance_of(owner_id.clone().unwrap_or(env::predecessor_account_id()))
            .unwrap_or_default();
        log!("Storage balance: {:?}", storage_balance);
        if storage_balance.total < storage_balance_bounds.min {
            // Deposit storage if necessary
            self.storage_deposit(
                Some(owner_id.clone().unwrap_or(env::predecessor_account_id())),
                None,
            );
        }

        Nep177Controller::mint_with_metadata(
            self,
            &token_id,
            &owner_id.unwrap_or(env::predecessor_account_id()),
            &metadata,
        )
        .unwrap_or_else(|e| env::panic_str(&e.to_string()));
    }
}
