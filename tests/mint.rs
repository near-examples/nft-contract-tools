pub mod common;

use near_sdk::serde_json::json;
use near_sdk_contract_tools::nft::Token;

const TOKEN_ID: &str = "id-0";

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
