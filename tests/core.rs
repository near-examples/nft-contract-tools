pub mod common;

use near_api::NearToken;
use near_sdk::serde_json::json;
use near_sdk_contract_tools::nft::Token;

const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);
const TOKEN_ID: &str = "id-0";

#[tokio::test]
async fn test_transfer_to_not_registered_account() -> testresult::TestResult<()> {
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

    // Get the token data
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the token is owned by the contract
    assert_eq!(
        token.owner_id.to_string(),
        nft_contract.account_id().to_string()
    );

    // Try to transfer the NFT usign not owner account
    nft_contract
        .call_function("nft_transfer", json!({"receiver_id": alice.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("simple transfer".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .with_signer(alice.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    Ok(())
}

#[tokio::test]
async fn test_transfer_to_registered_account() -> testresult::TestResult<()> {
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

    // Get the token data
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the token is owned by the contract
    assert_eq!(
        token.owner_id.to_string(),
        nft_contract.account_id().to_string()
    );

    // Register the alice account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &alice.account_id(),
    )
    .await?;

    // Transfer the NFT to the alice account
    nft_contract
        .call_function("nft_transfer", json!({"receiver_id": alice.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("simple transfer".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // Get the token data
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the token is owned by the alice account now
    assert_eq!(token.owner_id.to_string(), alice.account_id().to_string());

    Ok(())
}

#[tokio::test]
async fn test_transfer_call_fast_return_to_sender() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, token_receiver_contract, _, signer) =
        common::init_contracts(&sandbox, &sandbox_network).await?;

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

    // Register the token receiver contract account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &token_receiver_contract.account_id(),
    )
    .await?;

    // Transfer the NFT to the token receiver contract and call the return-it-now function
    nft_contract
        .call_function("nft_transfer_call", json!({"receiver_id": token_receiver_contract.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("transfer & call".to_string()), "msg": Some("return-it-now".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

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

#[tokio::test]
async fn test_transfer_call_slow_return_to_sender() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, token_receiver_contract, _, signer) =
        common::init_contracts(&sandbox, &sandbox_network).await?;

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

    // Register the token receiver contract account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &token_receiver_contract.account_id(),
    )
    .await?;

    // Transfer the NFT to the token receiver contract and call the return-it-later function
    nft_contract
        .call_function("nft_transfer_call", json!({"receiver_id": token_receiver_contract.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("transfer & call".to_string()), "msg": Some("return-it-later".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

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

#[tokio::test]
async fn test_transfer_call_fast_keep_with_sender() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, token_receiver_contract, _, signer) =
        common::init_contracts(&sandbox, &sandbox_network).await?;

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

    // Register the token receiver contract account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &token_receiver_contract.account_id(),
    )
    .await?;

    // Transfer the NFT to the token receiver contract and call the keep-it-now function
    nft_contract
        .call_function("nft_transfer_call", json!({"receiver_id": token_receiver_contract.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("transfer & call".to_string()), "msg": Some("keep-it-now".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // Get the token data
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the token is owned by the token receiver contract now
    assert_eq!(
        token.owner_id.to_string(),
        token_receiver_contract.account_id().to_string()
    );

    Ok(())
}

#[tokio::test]
async fn test_transfer_call_slow_keep_with_sender() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, token_receiver_contract, _, signer) =
        common::init_contracts(&sandbox, &sandbox_network).await?;

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

    // Register the token receiver contract account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &token_receiver_contract.account_id(),
    )
    .await?;

    // Transfer the NFT to the token receiver contract and call the keep-it-later function
    nft_contract
        .call_function("nft_transfer_call", json!({"receiver_id": token_receiver_contract.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("transfer & call".to_string()), "msg": Some("keep-it-later".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // Get the token data
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    // Check that the token is owned by the token receiver contract now
    assert_eq!(
        token.owner_id.to_string(),
        token_receiver_contract.account_id().to_string()
    );

    Ok(())
}

#[tokio::test]
async fn test_transfer_call_receiver_panics() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, token_receiver_contract, _, signer) =
        common::init_contracts(&sandbox, &sandbox_network).await?;

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

    // Register the token receiver contract account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &token_receiver_contract.account_id(),
    )
    .await?;

    // Transfer the NFT to the token receiver contract and pass an incorrect message to trigger a panic
    nft_contract
        .call_function("nft_transfer_call", json!({"receiver_id": token_receiver_contract.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("transfer & call".to_string()), "msg": Some("incorrect message".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .gas(near_sdk::Gas::from_gas(35_000_000_000_000 + 1))
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

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

#[tokio::test]
async fn test_transfer_call_receiver_panics_and_nft_resolve_transfer_produces_no_log_if_not_enough_gas()
-> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, token_receiver_contract, _, signer) =
        common::init_contracts(&sandbox, &sandbox_network).await?;

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

    // Register the token receiver contract account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &token_receiver_contract.account_id(),
    )
    .await?;

    // Transfer the NFT to the token receiver contract, pass an incorrect message to trigger a panic and set the gas limit to 3 TGas
    let res = nft_contract
        .call_function("nft_transfer_call", json!({"receiver_id": token_receiver_contract.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("transfer & call".to_string()), "msg": Some("incorrect message".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .gas(near_sdk::Gas::from_tgas(3))
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_failure();
    // Prints no logs
    assert_eq!(res.logs().len(), 0);

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

#[tokio::test]
async fn test_simple_transfer_no_logs_on_failure() -> testresult::TestResult<()> {
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

    // Transfer to the current owner should fail and not print log
    let res = nft_contract
        .call_function("nft_transfer", json!({"receiver_id": nft_contract.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("simple transfer".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .gas(near_sdk::Gas::from_tgas(200))
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    // Prints no logs
    assert_eq!(res.logs().len(), 0);

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
