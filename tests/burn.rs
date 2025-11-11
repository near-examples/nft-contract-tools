pub mod common;

use near_sdk::json_types::U128;
use near_sdk_contract_tools::nft::Token;
use near_workspaces::Worker;
use near_workspaces::network::Sandbox;
use near_workspaces::types::NearToken;

const TOKEN_ID: &str = "burn-0";
const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

#[tokio::test]
async fn test_burn_requires_one_yocto() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let (nft_contract, _, _) = common::init_contracts(&worker).await?;

    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        Some(nft_contract.id()),
    )
    .await?;

    let res = nft_contract
        .call("nft_burn")
        .args_json((TOKEN_ID,))
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_failure());

    let supply: U128 = nft_contract.call("nft_total_supply").view().await?.json()?;
    assert_eq!(supply, U128::from(1));

    Ok(())
}

#[tokio::test]
async fn test_burn_removes_token() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let (nft_contract, _, _) = common::init_contracts(&worker).await?;

    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        Some(nft_contract.id()),
    )
    .await?;

    let res = nft_contract
        .call("nft_burn")
        .args_json((TOKEN_ID,))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_success());

    let supply: U128 = nft_contract.call("nft_total_supply").view().await?.json()?;
    assert_eq!(supply, U128::from(0));

    let res = nft_contract
        .call("nft_token")
        .args_json((TOKEN_ID,))
        .view()
        .await?;
    assert!(res.json::<Option<Token>>()?.is_none());

    Ok(())
}

#[tokio::test]
async fn test_burn_fails_for_non_owner() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let alice = worker.dev_create_account().await?;
    let (nft_contract, _, _) = common::init_contracts(&worker).await?;

    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        Some(nft_contract.id()),
    )
    .await?;

    let res = alice
        .call(nft_contract.id(), "nft_burn")
        .args_json((TOKEN_ID,))
        .max_gas()
        .deposit(ONE_YOCTO)
        .transact()
        .await?;
    assert!(res.is_failure());

    let token: Token = nft_contract
        .call("nft_token")
        .args_json((TOKEN_ID,))
        .view()
        .await?
        .json()?;
    assert_eq!(token.owner_id.to_string(), nft_contract.id().to_string());

    Ok(())
}
