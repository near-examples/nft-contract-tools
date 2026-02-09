pub mod common;

use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_contract_tools::nft::Token;

#[tokio::test]
async fn test_enum_total_supply() -> testresult::TestResult<()> {
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
        "id-0".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    // Mint the another NFT
    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-1".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    // And one more NFT
    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-2".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    let total_supply: U128 = nft_contract
        .call_function("nft_total_supply", json!({}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(total_supply, U128::from(3));

    Ok(())
}

#[tokio::test]
async fn test_enum_nft_tokens() -> testresult::TestResult<()> {
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
        "id-0".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    // Mint the another NFT
    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-1".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    // And one more NFT
    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-2".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    // And one more NFT
    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-3".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    // No optional args should return all
    let mut tokens: Vec<Token> = nft_contract
        .call_function(
            "nft_tokens",
            json!({"from_index": Option::<U128>::None, "limit": Option::<u64>::None}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(tokens.len(), 4);

    // Start at "1", with no limit arg
    tokens = nft_contract
        .call_function(
            "nft_tokens",
            json!({"from_index": Some(U128::from(1)), "limit": Option::<u64>::None}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.get(0).unwrap().token_id, "id-1".to_string());
    assert_eq!(tokens.get(1).unwrap().token_id, "id-2".to_string());
    assert_eq!(tokens.get(2).unwrap().token_id, "id-3".to_string());

    // Start at "2", with limit 1
    tokens = nft_contract
        .call_function(
            "nft_tokens",
            json!({"from_index": Some(U128::from(2)), "limit": Some(1u64)}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens.get(0).unwrap().token_id, "id-2".to_string());

    // Don't specify from_index, but limit 2
    tokens = nft_contract
        .call_function(
            "nft_tokens",
            json!({"from_index": Option::<U128>::None, "limit": Some(2u64)}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens.get(0).unwrap().token_id, "id-0".to_string());
    assert_eq!(tokens.get(1).unwrap().token_id, "id-1".to_string());

    Ok(())
}

#[tokio::test]
async fn test_enum_nft_supply_for_owner() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Create a subaccount for the test
    let alice = common::create_subaccount(&sandbox, "alice.sandbox").await?;
    // Initialize the contracts
    let (nft_contract, _, _, signer) = common::init_contracts(&sandbox, &sandbox_network).await?;

    // Register the alice account
    common::register_user(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &alice.account_id(),
    )
    .await?;

    // Get number from account with no NFTs
    let owner_num_tokens: U128 = nft_contract
        .call_function(
            "nft_supply_for_owner",
            json!({"account_id": alice.account_id().clone()}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_num_tokens, U128::from(0));

    let owner_num_tokens: U128 = nft_contract
        .call_function(
            "nft_supply_for_owner",
            json!({"account_id": nft_contract.account_id().clone()}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_num_tokens, U128::from(0));

    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-0".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-1".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-2".into(),
        Some(&alice.account_id()),
    )
    .await?;

    let owner_num_tokens: U128 = nft_contract
        .call_function(
            "nft_supply_for_owner",
            json!({"account_id": nft_contract.account_id().clone()}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_num_tokens, U128::from(2));

    let alice_num_tokens: U128 = nft_contract
        .call_function(
            "nft_supply_for_owner",
            json!({"account_id": alice.account_id().clone()}),
        )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(alice_num_tokens, U128::from(1));

    Ok(())
}

#[tokio::test]
async fn test_enum_nft_tokens_for_owner() -> testresult::TestResult<()> {
    // Initialize the sandbox
    let (sandbox, sandbox_network) = common::init_sandbox().await?;
    // Create a subaccount for the test
    let alice = common::create_subaccount(&sandbox, "alice.sandbox").await?;
    // Initialize the contracts
    let (nft_contract, _, _, signer) = common::init_contracts(&sandbox, &sandbox_network).await?;

    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-0".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-1".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-2".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    common::mint_nft(
        &nft_contract,
        signer.clone(),
        &sandbox_network,
        &nft_contract.as_account(),
        "id-3".into(),
        Some(&nft_contract.account_id()),
    )
    .await?;

    // Get tokens from account with no NFTs
    let owner_tokens: Vec<Token> = nft_contract
        .call_function("nft_tokens_for_owner", json!({"account_id": alice.account_id().clone(), "from_index": Option::<U128>::None, "limit": Option::<u64>::None}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_tokens.len(), 0);

    // Get tokens with no optional args
    let owner_tokens: Vec<Token> = nft_contract
        .call_function("nft_tokens_for_owner", json!({"account_id": nft_contract.account_id().clone(), "from_index": Option::<U128>::None, "limit": Option::<u64>::None})   )
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_tokens.len(), 4);

    // With from_index and no limit
    let owner_tokens: Vec<Token> = nft_contract
        .call_function("nft_tokens_for_owner", json!({"account_id": nft_contract.account_id().clone(), "from_index": Some(U128::from(2)), "limit": Option::<u64>::None}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_tokens.len(), 2);
    assert_eq!(owner_tokens.get(0).unwrap().token_id, "id-2".to_string());
    assert_eq!(owner_tokens.get(1).unwrap().token_id, "id-3".to_string());

    // With from_index and limit 1
    let owner_tokens: Vec<Token> = nft_contract
        .call_function("nft_tokens_for_owner", json!({"account_id": nft_contract.account_id().clone(), "from_index": Some(U128::from(1)), "limit": Some(1u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_tokens.len(), 1);
    assert_eq!(owner_tokens.get(0).unwrap().token_id, "id-1".to_string());

    // No from_index but limit 3
    let owner_tokens: Vec<Token> = nft_contract
        .call_function("nft_tokens_for_owner", json!({"account_id": nft_contract.account_id().clone(), "from_index": Option::<U128>::None, "limit": Some(3u64)}))
        .read_only()
        .fetch_from(&sandbox_network)
        .await?
        .data;
    assert_eq!(owner_tokens.len(), 3);
    assert_eq!(owner_tokens.get(0).unwrap().token_id, "id-0".to_string());
    assert_eq!(owner_tokens.get(1).unwrap().token_id, "id-1".to_string());
    assert_eq!(owner_tokens.get(2).unwrap().token_id, "id-2".to_string());

    Ok(())
}
