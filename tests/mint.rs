pub mod common;

use near_sdk::serde_json::json;
use near_sdk_contract_tools::ft::{StorageBalance, StorageBalanceBounds};
use near_sdk_contract_tools::nft::Token;

const TOKEN_ID: &str = "id-0";

#[tokio::test]
async fn test_mint_registers_owner_without_storage() -> testresult::TestResult<()> {
    let (sandbox, network) = common::init_sandbox().await?;
    let alice = common::create_subaccount(&sandbox, "alice.sandbox").await?;
    let (contract, _, _, signer) = common::init_contracts(&sandbox, &network).await?;
    let before: Option<StorageBalance> = contract
        .call_function(
            "storage_balance_of",
            json!({"account_id": alice.account_id()}),
        )
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    assert!(before.is_none());

    common::mint_nft(
        &contract,
        signer,
        &network,
        &contract.as_account(),
        TOKEN_ID.into(),
        Some(&alice.account_id()),
    )
    .await?;

    let token: Token = contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    assert_eq!(token.owner_id, alice.account_id().clone());
    let after: Option<StorageBalance> = contract
        .call_function(
            "storage_balance_of",
            json!({"account_id": alice.account_id()}),
        )
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    let after = after.expect("mint should register and fund the recipient's storage");
    let bounds: StorageBalanceBounds = contract
        .call_function("storage_balance_bounds", json!({}))
        .read_only()
        .fetch_from(&network)
        .await?
        .data;
    assert!(after.total >= bounds.min);
    Ok(())
}

#[tokio::test]
async fn test_mint_defaults_owner_to_predecessor() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, _, _, signer) = common::init_contracts(&sandbox, &sandbox_network).await?;

    // Mint the NFT
    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        TOKEN_ID.into(),
        None,
    )
    .await?;

    // Get the token data
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the token is still owned by the contract
    assert_eq!(
        token.owner_id.to_string(),
        nft_contract.account_id().to_string()
    );

    Ok(())
}
