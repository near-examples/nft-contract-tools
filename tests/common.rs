use std::sync::{Arc, LazyLock};

use cargo_near_build::BuildOpts;
use near_api::{Account, Contract, NearToken, NetworkConfig, Signer};
use near_contract_standards::non_fungible_token::TokenId;

use near_sandbox::Sandbox;
use near_sdk::AccountId;
use near_sdk::json_types::U64;
use near_sdk::serde_json::json;
use near_sdk_contract_tools::nft::{ContractMetadata, TokenMetadata};

static NFT_CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let contract_wasm_path = cargo_near_build::build_with_cli(BuildOpts {
        no_abi: true,
        no_embed_abi: true,
        ..Default::default()
    })
    .expect("Could not compile NFT contract for tests");

    let contract_wasm = std::fs::read(contract_wasm_path.clone())
        .expect(format!("Could not read NFT WASM file from {}", contract_wasm_path).as_str());

    contract_wasm
});

static TOKEN_RECEIVER_CONTRACT_WASM: LazyLock<Vec<u8>> = LazyLock::new(|| {
    let contract_wasm_path = "tests/contracts/token-receiver/res/token_receiver.wasm";

    let contract_wasm = std::fs::read(contract_wasm_path).expect(
        format!(
            "Could not read Token Receiver Contract WASM file from {}",
            contract_wasm_path
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

pub async fn init_sandbox() -> anyhow::Result<(Sandbox, NetworkConfig)> {
    // Initialize the sandbox
    let sandbox = near_sandbox::Sandbox::start_sandbox().await?;
    let sandbox_network =
        near_api::NetworkConfig::from_rpc_url("sandbox", sandbox.rpc_addr.parse()?);

    Ok((sandbox, sandbox_network))
}
pub async fn init_contracts(
    sandbox: &Sandbox,
    sandbox_network: &NetworkConfig,
) -> anyhow::Result<(Contract, Contract, Contract, Arc<Signer>)> {
    let nft_contract = create_subaccount(&sandbox, "gbook.sandbox")
        .await
        .unwrap()
        .as_contract();
    let token_receiver_contract = create_subaccount(&sandbox, "token-receiver.sandbox")
        .await
        .unwrap()
        .as_contract();
    let approval_receiver_contract = create_subaccount(&sandbox, "approval-receiver.sandbox")
        .await
        .unwrap()
        .as_contract();
    // let token_receiver_contract = worker.dev_deploy(&TOKEN_RECEIVER_CONTRACT_WASM).await?;
    // let approval_receiver_contract = worker.dev_deploy(&APPROVAL_RECEIVER_CONTRACT_WASM).await?;
    // let nft_contract = worker.dev_deploy(&NFT_CONTRACT_WASM).await?;

    // Initialize signer for the contract deployment
    let signer = near_api::Signer::from_secret_key(
        near_sandbox::config::DEFAULT_GENESIS_ACCOUNT_PRIVATE_KEY
            .parse()
            .unwrap(),
    )?;

    // Initialize the contract metadata
    let metadata = ContractMetadata {
        spec: "nft-2.1.0".to_string(),
        name: "MyNftContract".to_string(),
        symbol: "MNFT".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
    };
    // Deploy the nft contract
    near_api::Contract::deploy(nft_contract.account_id().clone())
        .use_code(NFT_CONTRACT_WASM.to_vec())
        .with_init_call(
            "new",
            json!({"owner_id": nft_contract.account_id().clone(), "metadata": metadata}),
        )?
        .with_signer(signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();
    // Deploy the token receiver contract
    near_api::Contract::deploy(token_receiver_contract.account_id().clone())
        .use_code(TOKEN_RECEIVER_CONTRACT_WASM.to_vec())
        .without_init_call()
        .with_signer(signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();
    // Deploy the approval receiver contract
    near_api::Contract::deploy(approval_receiver_contract.account_id().clone())
        .use_code(APPROVAL_RECEIVER_CONTRACT_WASM.to_vec())
        .without_init_call()
        .with_signer(signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    Ok((
        nft_contract,
        token_receiver_contract,
        approval_receiver_contract,
        signer,
    ))
}

pub async fn register_user(
    contract: &Contract,
    signer: Arc<Signer>,
    sandbox_network: &NetworkConfig,
    account_id: &AccountId,
) -> anyhow::Result<()> {
    contract
        .call_function(
            "storage_deposit",
            json!({"account_id": account_id, "registration_only": Option::<bool>::None}),
        )
        .transaction()
        .deposit(NearToken::from_yoctonear(7000000000000000000000))
        .with_signer(contract.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();

    Ok(())
}

pub async fn mint_nft(
    contract: &Contract,
    signer: Arc<Signer>,
    sandbox_network: &NetworkConfig,
    minter: &Account,
    token_id: TokenId,
    token_owner_id: Option<&AccountId>,
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
    contract
        .call_function(
            "nft_mint",
            json!({"token_id": token_id, "metadata": token_metadata, "owner_id": Some(token_owner_id)}),
        )
        .transaction()
        .deposit(NearToken::from_millinear(21))
        .with_signer(minter.account_id().clone(), signer.clone())
        .send_to(&sandbox_network)
        .await?
        .assert_success();
    Ok(())
}

pub async fn create_subaccount(
    sandbox: &near_sandbox::Sandbox,
    name: &str,
) -> testresult::TestResult<near_api::Account> {
    let account_id: AccountId = name.parse().unwrap();
    sandbox
        .create_account(account_id.clone())
        .initial_balance(NearToken::from_near(10))
        .send()
        .await?;
    Ok(near_api::Account(account_id))
}
