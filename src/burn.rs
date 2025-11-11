use crate::{MyNftContract, MyNftContractExt};
use near_sdk::{assert_one_yocto, env, near};
use near_sdk_contract_tools::nft::{Nep177Controller, TokenId};

#[near]
impl MyNftContract {
    #[payable]
    pub fn nft_burn(&mut self, token_id: TokenId) {
        assert_one_yocto();

        Nep177Controller::burn_with_metadata(self, &token_id, &env::predecessor_account_id())
            .unwrap_or_else(|e| env::panic_str(&e.to_string()));
    }
}
