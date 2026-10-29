pub mod common;

use near_api::NearToken;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_contract_tools::nft::Token;

const TOKEN_ID: &str = "burn-0";
const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

#[tokio::test]
async fn test_burn_requires_one_yocto() -> testresult::TestResult<()> {
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
        Some(&nft_contract.account_id()),
    )
    .await?;

    nft_contract
        .call_function("nft_burn", json!({"token_id": TOKEN_ID}))
        .transaction()
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    let supply: U128 = nft_contract
        .call_function("nft_total_supply", json!({}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the total supply is 1
    assert_eq!(supply, U128::from(1));

    Ok(())
}

#[tokio::test]
async fn test_burn_removes_token() -> testresult::TestResult<()> {
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
        Some(&nft_contract.account_id()),
    )
    .await?;

    nft_contract
        .call_function("nft_burn", json!({"token_id": TOKEN_ID}))
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    let supply: U128 = nft_contract
        .call_function("nft_total_supply", json!({}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the total supply is 0
    assert_eq!(supply, U128::from(0));

    // Get the token data
    let token: Option<Token> = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(token, None);

    Ok(())
}

#[tokio::test]
async fn test_burn_fails_for_non_owner() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Create a subaccount for the test
    let alice = common::create_subaccount(&sandbox, "alice.sandbox").await?;
    // Initialize the contracts
    let (nft_contract, _, _, signer) = common::init_contracts(&sandbox, &sandbox_network).await?;

    // Mint the NFT
    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        TOKEN_ID.into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    nft_contract
        .call_function("nft_burn", json!({"token_id": TOKEN_ID}))
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(alice.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    // Get the token data
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(
        token.owner_id.to_string(),
        nft_contract.account_id().to_string()
    );

    Ok(())
}
