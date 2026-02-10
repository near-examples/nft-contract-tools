pub mod common;

use near_api::NearToken;
use near_sdk::serde_json::Value;
use near_sdk::serde_json::json;
use near_sdk_contract_tools::nft::Token;

pub const TOKEN_ID: &str = "0";

const ONE_NEAR: NearToken = NearToken::from_near(1);
const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

#[tokio::test]
pub async fn test_simple_approve() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Create a subaccount for the test
    let alice = common::create_subaccount(&sandbox, "alice.sandbox").await?;
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

    // root approves alice
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": alice.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // check nft_is_approved, don't provide approval_id
    let alice_approved: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": alice.account_id().clone(), "approval_id": Option::<u64>::None}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(alice_approved);

    // check nft_is_approved, with approval_id=0
    let alice_approval_id_is_0: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": alice.account_id().clone(), "approval_id": Some(0u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(alice_approval_id_is_0);

    // check nft_is_approved, with approval_id=1
    let alice_approval_id_is_1: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": alice.account_id().clone(), "approval_id": Some(1u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(!alice_approval_id_is_1);

    // check nft_is_approved, with approval_id=2
    let alice_approval_id_is_2: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": alice.account_id().clone(), "approval_id": Some(2u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(!alice_approval_id_is_2);

    // alternatively, one could check the data returned by nft_token
    let token: Token = nft_contract
        .call_function("nft_token", json!({"token_id": TOKEN_ID}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    let alice_approve_data = token
        .extensions_metadata
        .get("approved_account_ids")
        .unwrap()
        .as_object()
        .unwrap()
        .get_key_value(&alice.account_id().to_string())
        .unwrap();
    assert_eq!(
        alice_approve_data,
        (&alice.account_id().to_string(), &Value::Number(0.into()))
    );

    // can't approve the same account again
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": alice.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .max_gas()
        .deposit(ONE_NEAR)
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    // approving another account gives different approval_id
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": token_receiver_contract.account_id().clone(), "approval_id": Some(1u64)}))
        .transaction()
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .with_signer(nft_contract.account_id().clone(), signer)
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    let token_receiver_approval_id_is_1: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": token_receiver_contract.account_id().clone(), "approval_id": Some(1u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(token_receiver_approval_id_is_1);

    Ok(())
}

#[tokio::test]
pub async fn test_approval_with_call() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    let (nft_contract, _, approval_receiver_contract, signer) =
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

    nft_contract   .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": approval_receiver_contract.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    Ok(())
}

#[tokio::test]
pub async fn test_approval_with_call_and_different_msg() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Initialize the contracts
    let (nft_contract, _, approval_receiver_contract, signer) =
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

    // The approval_receiver implementation will return given `msg` after subsequent promise call,
    // if given something other than "return-now".
    let msg = "hahaha".to_string();
    let res = nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": approval_receiver_contract.account_id().clone(), "msg": Some(msg.clone())}))
        .transaction()
        .max_gas()
        .deposit(ONE_YOCTO)
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?;

    assert_eq!(res.json::<String>()?, msg);
    Ok(())
}

#[tokio::test]
pub async fn test_approved_account_transfers_token() -> testresult::TestResult<()> {
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

    // Register the alice account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &alice.account_id(),
    )
    .await?;

    // root approves alice
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": alice.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // alice sends to self
    nft_contract
        .call_function("nft_transfer", json!({"receiver_id": alice.account_id().clone(), "token_id": TOKEN_ID, "approval_id": Some(0u64), "memo": Some("gotcha! bahahaha".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(alice.account_id().clone(), signer.clone())
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
pub async fn test_revoke() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Create a subaccount for the test
    let alice = common::create_subaccount(&sandbox, "alice.sandbox").await?;
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

    // root approves alice
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": alice.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // root approves token_receiver
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": token_receiver_contract.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // root revokes alice
    nft_contract
        .call_function(
            "nft_revoke",
            json!({"token_id": TOKEN_ID, "account_id": alice.account_id().clone()}),
        )
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // alice is revoked...
    let alice_approved: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": alice.account_id().clone(), "approval_id": Some(0u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(!alice_approved);

    // but token_receiver is still approved
    let token_receiver_approved: bool = nft_contract
          .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": token_receiver_contract.account_id().clone(), "approval_id": Option::<u64>::None}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(token_receiver_approved);

    // root revokes token_receiver
    nft_contract
        .call_function("nft_revoke", json!({"token_id": TOKEN_ID, "account_id": token_receiver_contract.account_id().clone()}))
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // alice is still revoked...
    let alice_approved: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": alice.account_id().clone(), "approval_id": Some(0u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(!alice_approved);

    // ...and now so is token_receiver
    let token_receiver_approved: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": token_receiver_contract.account_id().clone(), "approval_id": Option::<u64>::None}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(!token_receiver_approved);

    // alice tries to send it to self and fails
    nft_contract
        .call_function("nft_transfer", json!({"receiver_id": alice.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("gotcha! bahahaha".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .gas(near_sdk::Gas::from_gas(200))
        .with_signer(alice.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    Ok(())
}

#[tokio::test]
pub async fn test_revoke_all() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Create a subaccount for the test
    let alice = common::create_subaccount(&sandbox, "alice.sandbox").await?;
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

    // root approves alice
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": alice.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // root approves token_receiver
    nft_contract
        .call_function("nft_approve", json!({"token_id": TOKEN_ID, "account_id": token_receiver_contract.account_id().clone(), "approval_id": Option::<String>::None}))
        .transaction()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // root revokes all
    nft_contract
        .call_function("nft_revoke_all", json!({"token_id": TOKEN_ID}))
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(nft_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    // alice is revoked...
    let alice_approved: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": alice.account_id().clone(), "approval_id": Some(0u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(!alice_approved);

    // and so is token_receiver
    let token_receiver_approved: bool = nft_contract
        .call_function("nft_is_approved", json!({"token_id": TOKEN_ID, "approved_account_id": token_receiver_contract.account_id().clone(), "approval_id": Option::<u64>::None}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert!(!token_receiver_approved);

    // alice tries to send it to self and fails
    nft_contract
        .call_function("nft_transfer", json!({"receiver_id": alice.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("gotcha! bahahaha".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(alice.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    // so does token_receiver
    nft_contract
        .call_function("nft_transfer", json!({"receiver_id": alice.account_id().clone(), "token_id": TOKEN_ID, "memo": Some("gotcha! bahahaha".to_string())}))
        .transaction()
        .deposit(ONE_YOCTO)
        .max_gas()
        .with_signer(token_receiver_contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_failure();

    Ok(())
}
