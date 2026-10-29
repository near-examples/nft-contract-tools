# Non-fungible Token (NFT) based on Contract Tools 🖼️ 

## Rust toolchain

The contracts use NEAR SDK 5.29.1 and the pinned Rust 1.97.1 toolchain. Install [rustup](https://rustup.rs/) and `cargo-near` before building or running the local sandbox integration tests:

```bash
cargo install cargo-near --version 0.21.1 --locked --no-default-features
```

Contract builds and both receiver fixtures compile from source with their committed lockfiles. Integration tests use `near-sandbox` and `near-api` against a local RPC endpoint. A maximum of two sandboxes run concurrently per test binary to avoid setup timeouts. No live network account is required for tests.


[![](https://img.shields.io/badge/⋈%20Examples-Basics-green)](https://docs.near.org/tutorials/welcome)
[![](https://img.shields.io/badge/Contract-Rust-red)](contract-rs)

This repository contains an example implementation of a non-fungible token contract in Rust which uses [near-sdk-contract-tools](https://github.com/near/near-sdk-contract-tools).

- `non-fungible token`: https://nomicon.io/Standards/NonFungibleToken/README.html
- `near-sdk-contract-tools`: https://github.com/near/near-sdk-contract-tools
- `near-contract-standards`: https://github.com/near/near-sdk-rs/tree/master/near-contract-standards
- `near-sandbox`: https://github.com/near/near-sandbox-rs
- `near-api`: https://github.com/near/near-api-rs

>**Note**: If you'd like to learn how to create an NFT contract from scratch that explores every aspect of the [NEP-171](https://github.com/near/NEPs/blob/master/neps/nep-0171.md) standard including an NFT marketplace, check out the NFT [Zero to Hero Tutorial](https://docs.near.org/tutorials/nfts/introduction).

<br />

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

> Note: to avoid issues, be sure to update your Rust compiler with `rustup update stable`

## How to Test Locally?

```bash
cargo test --locked
```

## How to Deploy?

To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# Deploy the contract and initialize it
cargo near deploy build-non-reproducible-wasm <account-id> with-init-call new json-args '{"owner_id":"<account-id>", "metadata": {"name": "GO TEAM", "symbol": "GOAT", "icon": "https://bafybeidl4hjbpdr6u6xvlrizwxbrfcyqurzvcnn5xoilmcqbxfbdwrmp5m.ipfs.dweb.link/", "spec": "nft-1.0.0"}}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

## Basic methods
```bash
# View metadata
near view <account-id> nft_metadata

# Mint a NFT
near call <account-id> nft_mint '{"token_id": "0", "owner_id": "<account-id>", "metadata": {"title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": "1"}}' --accountId <account-id> --deposit 0.1

# View tokens for owner
near view <account-id> nft_tokens_for_owner '{"account_id": "<owner_id>"}'

# Transfer a NFT
near call <account-id> nft_transfer '{"token_id": "0", "receiver_id": "<receiver-id>", "memo": "transfer ownership"}' --accountId <account-id> --depositYocto 1
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Iteract with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NFT Zero to Hero Tutorial](https://docs.near.org/tutorials/nfts/introduction)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
