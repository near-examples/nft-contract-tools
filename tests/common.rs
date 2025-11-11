use std::sync::LazyLock;

use cargo_near_build::BuildOpts;
use near_contract_standards::non_fungible_token::TokenId;

use near_sdk::AccountId;
use near_sdk::json_types::U64;
use near_sdk::serde_json::json;
use near_sdk_contract_tools::nft::{ContractMetadata, TokenMetadata};
use near_workspaces::types::NearToken;
use near_workspaces::{Account, Contract, DevNetwork, Worker};

static NFT_CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let artifact = cargo_near_build::build(BuildOpts {
        no_abi: true,
        no_embed_abi: true,
        ..Default::default()
    })
    .expect("Could not compile NFT contract for tests");

    let contract_wasm = std::fs::read(&artifact.path)
        .expect(format!("Could not read NFT WASM file from {}", artifact.path).as_str());

    contract_wasm
});

static TOKEN_RECEIVER_CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let artifact_path = "tests/contracts/token-receiver/res/token_receiver.wasm";

    let contract_wasm = std::fs::read(artifact_path).expect(
        format!(
            "Could not read Token Receiver Contract WASM file from {}",
            artifact_path
        )
        .as_str(),
    );

    contract_wasm
});

static APPROVAL_RECEIVER_CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let artifact_path = "tests/contracts/approval-receiver/res/approval_receiver.wasm";

    let contract_wasm = std::fs::read(artifact_path).expect(
        format!(
            "Could not read Approval Receiver Contract WASM file from {}",
            artifact_path
        )
        .as_str(),
    );

    contract_wasm
});

pub async fn init_contracts(
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<(Contract, Contract, Contract)> {
    let nft_contract = worker.dev_deploy(&NFT_CONTRACT_WASM).await?;

    let metadata = ContractMetadata {
        spec: "nft-2.1.0".to_string(),
        name: "MyNftContract".to_string(),
        symbol: "MNFT".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
    };
    let res = nft_contract
        .call("new")
        .args_json(json!({"owner_id": nft_contract.id(), "metadata": metadata}))
        .max_gas()
        .transact()
        .await?;
    assert!(res.is_success());

    let token_receiver_contract = worker.dev_deploy(&TOKEN_RECEIVER_CONTRACT_WASM).await?;
    let approval_receiver_contract = worker.dev_deploy(&APPROVAL_RECEIVER_CONTRACT_WASM).await?;

    Ok((
        nft_contract,
        token_receiver_contract,
        approval_receiver_contract,
    ))
}

pub async fn register_user(contract: &Contract, account_id: &AccountId) -> anyhow::Result<()> {
    let res = contract
        .call("storage_deposit")
        .args_json((account_id, Option::<bool>::None))
        .max_gas()
        .deposit(NearToken::from_yoctonear(7000000000000000000000))
        .transact()
        .await?;
    assert!(res.is_success());

    Ok(())
}

pub async fn mint_nft(
    minter: &Account,
    contract_id: &AccountId,
    token_id: TokenId,
    token_owner_id: &AccountId,
) -> anyhow::Result<()> {
    let token_metadata = TokenMetadata {
        title: Some(format!("Title for {token_id}")),
        description: Some(format!("Description for {token_id}")),
        media: None,
        media_hash: None,
        copies: Some(U64::from(1)),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    };
    let res = minter
        .call(contract_id, "nft_mint")
        .args_json(
            json!({"token_id": token_id, "metadata": token_metadata, "owner_id": Some(token_owner_id)}),
        )
        .max_gas()
        .deposit(NearToken::from_millinear(21))
        .transact()
        .await?;
    println!("Mint NFT: {:?}", res);
    assert!(res.is_success());

    Ok(())
}
