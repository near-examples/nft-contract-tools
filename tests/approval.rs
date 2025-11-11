pub mod common;

use near_sdk::serde_json::Value;
use near_sdk_contract_tools::nft::Token;
use near_workspaces::Worker;
use near_workspaces::network::Sandbox;
use near_workspaces::{AccountId, types::NearToken};

use std::collections::HashMap;
use std::convert::TryFrom;

pub const TOKEN_ID: &str = "0";

const ONE_NEAR: NearToken = NearToken::from_near(1);
const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

// #[tokio::test]
// async fn approval() -> anyhow::Result<()> {
//     let nft_wasm = near_workspaces::compile_project(".").await.unwrap();
//     let token_receiver_wasm = near_workspaces::compile_project("./tests/contracts/token-receiver")
//         .await
//         .unwrap();
//     let approval_receiver_wasm =
//         near_workspaces::compile_project("./tests/contracts/approval-receiver")
//             .await
//             .unwrap();
//     let worker: near_workspaces::Worker<near_workspaces::network::Sandbox> =
//         near_workspaces::sandbox().await?;

//     let simple_approval = test_simple_approve(&worker, &nft_wasm, &token_receiver_wasm);
//     let approval_with_call = test_approval_with_call(&worker, &nft_wasm, &approval_receiver_wasm);
//     let approved_account_transfers_token =
//         test_approved_account_transfers_token(&worker, &nft_wasm);
//     let revoke = test_revoke(&worker, &nft_wasm, &token_receiver_wasm);
//     let revoke_all = test_revoke_all(&worker, &nft_wasm, &token_receiver_wasm);

//     // make sure they all pass
//     simple_approval.await?;
//     approval_with_call.await?;
//     approved_account_transfers_token.await?;
//     revoke.await?;
//     revoke_all.await?;

//     Ok(())
// }

#[tokio::test]
pub async fn test_simple_approve() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let alice = worker.dev_create_account().await?;
    let (nft_contract, token_receiver_contract, _) = common::init_contracts(&worker).await?;

    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        nft_contract.id(),
    )
    .await?;

    // common::register_user(&nft_contract, &alice.id()).await?;

    // root approves alice
    let res = nft_contract
        .call("nft_approve")
        .args_json((TOKEN_ID, alice.id(), Option::<String>::None))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    // check nft_is_approved, don't provide approval_id
    let alice_approved = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, alice.id(), Option::<u64>::None))
        .view()
        .await?
        .json::<bool>()?;
    assert!(alice_approved);

    // check nft_is_approved, with approval_id=0
    let alice_approval_id_is_0 = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, alice.id(), Some(0u64)))
        .view()
        .await?
        .json::<bool>()?;
    assert!(alice_approval_id_is_0);

    // check nft_is_approved, with approval_id=1
    let alice_approval_id_is_1 = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, alice.id(), Some(1u64)))
        .view()
        .await?
        .json::<bool>()?;
    assert!(!alice_approval_id_is_1);

    // check nft_is_approved, with approval_id=2
    let alice_approval_id_is_2 = nft_contract
        .call("nft_is_approved")
        .args_json(&(TOKEN_ID, alice.id(), Some(2u64)))
        .view()
        .await?
        .json::<bool>()?;
    assert!(!alice_approval_id_is_2);

    // alternatively, one could check the data returned by nft_token
    let token = nft_contract
        .call("nft_token")
        .args_json((TOKEN_ID,))
        .view()
        .await?
        .json::<Token>()?;
    let alice_approve_data = token
        .extensions_metadata
        .get("approved_account_ids")
        .unwrap()
        .as_object()
        .unwrap()
        .get_key_value(&alice.id().to_string())
        .unwrap();
    assert_eq!(
        alice_approve_data,
        (&alice.id().to_string(), &Value::Number(0.into()))
    );

    // can't approve the same account again
    let res = nft_contract
        .call("nft_approve")
        .args_json((TOKEN_ID, alice.id(), Option::<String>::None))
        .max_gas()
        .deposit(ONE_NEAR)
        .transact()
        .await?;
    assert!(res.is_failure());

    // approving another account gives different approval_id
    let res = nft_contract
        .call("nft_approve")
        .args_json((
            TOKEN_ID,
            token_receiver_contract.id(),
            Option::<String>::None,
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    let token_receiver_approval_id_is_1 = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, token_receiver_contract.id(), Some(1u64)))
        .view()
        .await?
        .json::<bool>()?;
    assert!(token_receiver_approval_id_is_1);

    Ok(())
}

#[tokio::test]
pub async fn test_approval_with_call() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let (nft_contract, _, approval_receiver_contract) = common::init_contracts(&worker).await?;
    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        nft_contract.id(),
    )
    .await?;

    let res = nft_contract
        .call("nft_approve")
        .args_json((
            TOKEN_ID,
            approval_receiver_contract.id(),
            Some("return-now".to_string()),
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert_eq!(res.json::<String>()?, "cool".to_string());

    Ok(())
}

#[tokio::test]
pub async fn test_approval_with_call_and_different_msg() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let (nft_contract, _, approval_receiver_contract) = common::init_contracts(&worker).await?;
    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        nft_contract.id(),
    )
    .await?;

    // The approval_receiver implementation will return given `msg` after subsequent promise call,
    // if given something other than "return-now".
    let msg = "hahaha".to_string();
    let res = nft_contract
        .call("nft_approve")
        .args_json((TOKEN_ID, approval_receiver_contract.id(), Some(msg.clone())))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert_eq!(res.json::<String>()?, msg);

    Ok(())
}

#[tokio::test]
pub async fn test_approved_account_transfers_token() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let alice = worker.dev_create_account().await?;
    let (nft_contract, _, _) = common::init_contracts(&worker).await?;
    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        nft_contract.id(),
    )
    .await?;
    common::register_user(&nft_contract, &alice.id()).await?;

    // root approves alice
    let res = nft_contract
        .call("nft_approve")
        .args_json((TOKEN_ID, alice.id(), Option::<String>::None))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    // alice sends to self
    let res = alice
        .call(nft_contract.id(), "nft_transfer")
        .args_json((
            alice.id(),
            TOKEN_ID,
            Some(0u64),
            Some("gotcha! bahahaha".to_string()),
        ))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_success());

    // token now owned by alice
    let token = nft_contract
        .call("nft_token")
        .args_json((TOKEN_ID,))
        .view()
        .await?
        .json::<Token>()?;
    assert_eq!(token.owner_id.to_string(), alice.id().to_string());

    Ok(())
}

#[tokio::test]
pub async fn test_revoke() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let alice = worker.dev_create_account().await?;
    let (nft_contract, token_receiver_contract, _) = common::init_contracts(&worker).await?;
    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        nft_contract.id(),
    )
    .await?;

    // root approves alice
    let res = nft_contract
        .call("nft_approve")
        .args_json((TOKEN_ID, alice.id(), Option::<String>::None))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    // root approves token_receiver
    let res = nft_contract
        .call("nft_approve")
        .args_json((
            TOKEN_ID,
            token_receiver_contract.id(),
            Option::<String>::None,
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    // root revokes alice
    let res = nft_contract
        .call("nft_revoke")
        .args_json((TOKEN_ID, alice.id()))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_success());

    // alice is revoked...
    let alice_approved = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, alice.id(), Some(0u64)))
        .view()
        .await?
        .json::<bool>()?;
    assert!(!alice_approved);

    // but token_receiver is still approved
    let token_receiver_approved = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, token_receiver_contract.id(), Option::<u64>::None))
        .view()
        .await?
        .json::<bool>()?;
    assert!(token_receiver_approved);

    // root revokes token_receiver
    let res = nft_contract
        .call("nft_revoke")
        .args_json((TOKEN_ID, token_receiver_contract.id()))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_success());

    // alice is still revoked...
    let alice_approved = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, alice.id(), Some(0u64)))
        .view()
        .await?
        .json::<bool>()?;
    assert!(!alice_approved);

    // ...and now so is token_receiver
    let token_receiver_approved = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, token_receiver_contract.id(), Option::<u64>::None))
        .view()
        .await?
        .json::<bool>()?;
    assert!(!token_receiver_approved);

    // alice tries to send it to self and fails
    let res = alice
        .call(nft_contract.id(), "nft_transfer")
        .args_json((
            alice.id(),
            TOKEN_ID,
            Some(0u64),
            Some("gotcha! bahahaha".to_string()),
        ))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_failure());

    Ok(())
}

#[tokio::test]
pub async fn test_revoke_all() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let alice = worker.dev_create_account().await?;
    let (nft_contract, token_receiver_contract, _) = common::init_contracts(&worker).await?;
    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        nft_contract.id(),
    )
    .await?;

    // root approves alice
    let res = nft_contract
        .call("nft_approve")
        .args_json((TOKEN_ID, alice.id(), Option::<String>::None))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    // root approves token_receiver
    let res = nft_contract
        .call("nft_approve")
        .args_json((
            TOKEN_ID,
            token_receiver_contract.id(),
            Option::<String>::None,
        ))
        .max_gas()
        .deposit(NearToken::from_yoctonear(550000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    // root revokes all
    let res = nft_contract
        .call("nft_revoke_all")
        .args_json((TOKEN_ID,))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_success());

    // alice is revoked...
    let alice_approved = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, alice.id(), Some(0u64)))
        .view()
        .await?
        .json::<bool>()?;
    assert!(!alice_approved);

    // and so is token_receiver
    let token_receiver_approved = nft_contract
        .call("nft_is_approved")
        .args_json((TOKEN_ID, token_receiver_contract.id(), Option::<u64>::None))
        .view()
        .await?
        .json::<bool>()?;
    assert!(!token_receiver_approved);

    // alice tries to send it to self and fails
    let res = alice
        .call(nft_contract.id(), "nft_transfer")
        .args_json((
            alice.id(),
            TOKEN_ID,
            Some(0u64),
            Some("gotcha! bahahaha".to_string()),
        ))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_failure());

    // so does token_receiver
    let res = token_receiver_contract
        .as_account()
        .call(nft_contract.id(), "nft_transfer")
        .args_json((
            alice.id(),
            TOKEN_ID,
            Some(1u64),
            Some("gotcha! bahahaha".to_string()),
        ))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_failure());

    Ok(())
}
