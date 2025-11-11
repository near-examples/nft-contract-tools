pub mod common;

use near_sdk_contract_tools::ft::{StorageBalance, StorageBalanceBounds};
use near_sdk_contract_tools::nft::Token;
use near_workspaces::Worker;
use near_workspaces::network::Sandbox;

const TOKEN_ID: &str = "id-0";

#[tokio::test]
async fn test_mint_registers_owner_without_storage() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let alice = worker.dev_create_account().await?;
    let (nft_contract, _, _) = common::init_contracts(&worker).await?;

    let storage_before: Option<StorageBalance> = nft_contract
        .call("storage_balance_of")
        .args_json((alice.id(),))
        .view()
        .await?
        .json()?;
    assert!(storage_before.is_none());

    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        Some(alice.id()),
    )
    .await?;

    let token: Token = nft_contract
        .call("nft_token")
        .args_json((TOKEN_ID,))
        .view()
        .await?
        .json()?;
    assert_eq!(token.owner_id.to_string(), alice.id().to_string());

    let storage_after: Option<StorageBalance> = nft_contract
        .call("storage_balance_of")
        .args_json((alice.id(),))
        .view()
        .await?
        .json()?;
    let storage_after = storage_after.expect("storage should be deposited");

    let bounds: StorageBalanceBounds = nft_contract
        .call("storage_balance_bounds")
        .view()
        .await?
        .json()?;
    assert!(
        storage_after.total >= bounds.min,
        "expected storage total {} to be at least {}",
        storage_after.total,
        bounds.min
    );

    Ok(())
}

#[tokio::test]
async fn test_mint_defaults_owner_to_predecessor() -> anyhow::Result<()> {
    let worker: Worker<Sandbox> = near_workspaces::sandbox().await?;
    let (nft_contract, _, _) = common::init_contracts(&worker).await?;

    common::mint_nft(
        nft_contract.as_account(),
        nft_contract.id(),
        TOKEN_ID.into(),
        None,
    )
    .await?;

    let token: Token = nft_contract
        .call("nft_token")
        .args_json((TOKEN_ID,))
        .view()
        .await?
        .json()?;
    assert_eq!(token.owner_id.to_string(), nft_contract.id().to_string());

    Ok(())
}
