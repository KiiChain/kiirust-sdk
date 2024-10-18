#![cfg(feature = "integration")]

use cosmrs::{crypto::secp256k1, dev, rpc, tx::AccountNumber, Coin};
use rwa_sdk::token::request::{TokenInfoRequest, TransferMessageRequest};
use rwa_sdk::RwaClient;
use std::panic::AssertUnwindSafe;
use std::str::FromStr;

// Constants
const CHAIN_ID: &str = "rwa-test";
const RPC_PORT: u16 = 26657;
const ACCOUNT_NUMBER: AccountNumber = 1;
const ACCOUNT_PREFIX: &str = "cosmos";
const DENOM: &str = "urwa";

#[tokio::test]
async fn test_token_transfer() {
    let sender_private_key = secp256k1::SigningKey::random();
    let sender_public_key = sender_private_key.public_key();
    let sender_account_id = sender_public_key.account_id(ACCOUNT_PREFIX).unwrap();

    let recipient_private_key = secp256k1::SigningKey::random();
    let recipient_account_id = recipient_private_key
        .public_key()
        .account_id(ACCOUNT_PREFIX)
        .unwrap();

    let amount = Coin {
        amount: 100u8.into(),
        denom: DENOM.parse().unwrap(),
    };

    let docker_args = [
        "-d",
        "-p",
        &format!("{}:{}", RPC_PORT, RPC_PORT),
        dev::GAIA_DOCKER_IMAGE,
        CHAIN_ID,
        &sender_account_id.to_string(),
    ];

    // Wrap the SigningKey in AssertUnwindSafe
    let sender_private_key = AssertUnwindSafe(sender_private_key);

    dev::docker_run(&docker_args, || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let rpc_address = format!("http://localhost:{}", RPC_PORT);
            let rpc_client = rpc::HttpClient::new(rpc_address.as_str()).unwrap();

            dev::poll_for_first_block(&rpc_client).await;

            // Initialize RwaClient
            let client = RwaClient::new(
                &rpc_address,
                CHAIN_ID,
                "cosmos1token...",      // Replace with actual token address
                "cosmos1identity...",   // Replace with actual identity address
                "cosmos1compliance...", // Replace with actual compliance address
            )
            .unwrap();

            // Perform token transfer
            let transfer_request = TransferMessageRequest {
                from: sender_account_id.to_string(),
                to: recipient_account_id.to_string(),
                amount: amount.amount.u128(),
                signer: (*sender_private_key).clone(),
            };

            let transfer_result = client.transfer(transfer_request).await.unwrap();
            println!("Transfer hash: {}", transfer_result);

            // Verify transfer
            let tx = dev::poll_for_tx(&rpc_client, transfer_result.parse().unwrap()).await;
            assert!(tx.auth_info.fee.amount.len() > 0);

            // Check recipient balance
            let balance_request = TokenInfoRequest {
                address: recipient_account_id.to_string(),
            };
            let balance = client.balance(balance_request).await.unwrap();
            assert_eq!(balance.balance, amount.amount.into());

            // Check token info
            let token_info = client.coin_info().await.unwrap();
            println!(
                "Token name: {}, symbol: {}",
                token_info.name, token_info.symbol
            );
        })
    });
}
