//! # RWA SDK
//!
//! The RWA (Real World Asset) SDK is a Rust library for interacting with tokenized
//! real-world assets on Cosmos-based blockchains. It provides functionality for
//! token operations, identity management, and compliance handling.
//!
//! ## Features
//!
//! - Token transfers and balance checks
//! - Identity registration and management
//! - Compliance module integration
//! - Blockchain interaction via RPC
//!
//! ## Usage Example
//!
//! ```rust
//! use rwa_sdk::RwaClient;
//! use cosmrs::crypto::secp256k1::SigningKey;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize the client
//!     let client = RwaClient::new(
//!         "http://rpc.example.com:26657",
//!         "my-chain-id",
//!         "cosmos1token...",
//!         "cosmos1identity...",
//!         "cosmos1compliance...",
//!         "sei",
//!         "gas_price"
//!
//!     )?;
//!
//!     // Perform a token transfer
//!     let signer = SigningKey::from_slice(&[/* your private key */])?;
//!     let transfer_result = client.transfer(TransferMessageRequest {
//!         from: "cosmos1sender...".to_string(),
//!         to: "cosmos1recipient...".to_string(),
//!         amount: 100,
//!         signer,
//!         gas_limit
//!     }).await?;
//!     println!("Transfer hash: {}", transfer_result.hash);
//!
//!     // Check a balance
//!     let balance = client.balance(TokenInfoRequest {
//!         address: "cosmos1address...".to_string(),
//!     }).await?;
//!     println!("Balance: {}", balance.balance);
//!
//!     Ok(())
//! }
//! ```
//!
//! This example demonstrates how to initialize the `RwaClient`, perform a token
//! transfer, and check an account balance. Error handling and proper setup of the
//! signing key are crucial for production use.
//!
//! For more detailed information on each function and module, please refer to their
//! respective documentation.

use cosmrs::proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::proto::cosmos::base::tendermint::v1beta1::AbciQueryResponse;
use cosmrs::proto::prost::Message;
use cosmrs::rpc::HttpClient;
use cosmrs::tendermint::abci::Event;
use cosmrs::{
    proto::cosmwasm::wasm::v1::MsgExecuteContract,
    rpc::Client,
    tendermint::chain::Id,
    tx::{self, Fee, MessageExt, SignDoc, SignerInfo},
    AccountId, Coin,
};
use cosmrs::{Any, Gas};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod compliance;
pub mod identity;
pub mod token;

#[derive(Debug, Clone)]
pub struct RwaClient {
    rpc_client: HttpClient,
    chain_id: String,
    token_address: String,
    identity_address: String,
    compliance_address: String,
    denom: String,
    gas_price: Gas,
}

struct AccountInfoResponse {
    pub account_number: u64,
    pub sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteResponse {
    /// The transaction hash
    pub tx_hash: String,
    /// The response data from the contract execution
    pub data: Vec<u8>,
    /// Gas used by the transaction
    pub gas_used: i64,
    /// Gas wanted/requested for the transaction
    pub gas_wanted: i64,
    /// Events emitted during execution
    pub events: Vec<Event>,
    /// Height of the block where this transaction was committed
    pub height: u64,
}

impl RwaClient {
    /// Creates a new RwaClient instance.
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The URL of the RPC endpoint
    /// * `chain_id` - The ID of the blockchain
    /// * `token_address` - The address of the token contract
    /// * `identity_address` - The address of the identity contract
    /// * `compliance_address` - The address of the compliance contract
    /// * `denom` - The unit of token
    /// * `gas_price` - The amount willing to pay for each unit of gas
    /// # Returns
    ///
    /// A Result containing the RwaClient instance or an error
    pub fn new(
        rpc_url: &str,
        chain_id: &str,
        token_address: &str,
        identity_address: &str,
        compliance_address: &str,
        denom: &str,
        gas_price: Gas,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let rpc_client = HttpClient::new(rpc_url)?;

        Ok(Self {
            rpc_client,
            chain_id: chain_id.to_string(),
            token_address: token_address.to_string(),
            identity_address: identity_address.to_string(),
            compliance_address: compliance_address.to_string(),
            denom: denom.to_string(),
            gas_price,
        })
    }

    /// Executes a contract call that modifies the state.
    ///
    /// # Arguments
    ///
    /// * `from` - The address initiating the transaction
    /// * `msg` - The message to be executed
    /// * `contract_address` - The address of the contract to execute
    /// * `funds` - Any funds to be sent with the transaction
    /// * `signer` - The signing key for the transaction
    ///
    /// # Returns
    ///
    /// A Result containing the transaction hash as a String or an error
    async fn execute<T: serde::Serialize>(
        &self,
        from: &str,
        msg: &T,
        contract_address: String,
        funds: Vec<Coin>,
        signer: &cosmrs::crypto::secp256k1::SigningKey,
        gas_limit: Gas,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let execute_msg = MsgExecuteContract {
            sender: from.to_string(),
            contract: contract_address,
            msg: cosmwasm_std::to_json_binary(msg)?.into(),
            funds: funds.into_iter().map(|c| c.into()).collect(),
        };

        let type_url = "/cosmwasm.wasm.v1.MsgExecuteContract".to_string();
        let value = execute_msg.to_bytes()?;
        let any_msg = cosmrs::Any { type_url, value };

        let tx_body = tx::BodyBuilder::new().msg(any_msg).finish();

        let sender_account_id = AccountId::from_str(from)?;
        let account_info = self.fetch_account_info(&sender_account_id).await?;

        // Calculate fee based on user-specified gas limit
        let fee_amount = gas_limit * self.gas_price;
        let fee = Fee::from_amount_and_gas(
            Coin {
                amount: fee_amount.into(),
                denom: self.denom.parse()?,
            },
            gas_limit,
        );

        // Prepare authentication info
        let auth_info = SignerInfo::single_direct(Some(signer.public_key()), account_info.sequence)
            .auth_info(fee);

        // Construct the sign doc
        let chain_id = Id::from_str(&self.chain_id)?;
        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_info.account_number)?;

        let tx_raw = sign_doc.sign(signer)?;

        let tx_bytes = tx_raw.to_bytes()?;

        let response = self.rpc_client.broadcast_tx_commit(tx_bytes).await?;

        // Convert events from the response
        let events: Vec<Event> = response
            .tx_result
            .events
            .into_iter()
            .map(|evt| Event {
                kind: evt.kind,
                attributes: evt.attributes,
            })
            .collect();

        Ok(ExecuteResponse {
            tx_hash: response.hash.to_string(),
            data: response.tx_result.data.to_vec(),
            gas_used: response.check_tx.gas_used,
            gas_wanted: response.tx_result.gas_wanted,
            events,
            height: response.height.value(),
        })
    }

    /// Queries a contract without modifying the state.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The address of the contract to query
    /// * `msg` - The query message
    ///
    /// # Returns
    ///
    /// A Result containing the deserialized response or an error
    async fn query<T: serde::de::DeserializeOwned>(
        &self,
        contract_address: &str,
        msg: &impl serde::Serialize,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let query_msg = cosmwasm_std::to_json_binary(&msg)?;
        let query_data = cosmrs::proto::cosmwasm::wasm::v1::QuerySmartContractStateRequest {
            address: contract_address.to_string(),
            query_data: query_msg.into(),
        };
        let query_data = query_data.encode_to_vec();

        let path = "/cosmwasm.wasm.v1.Query/SmartContractState";

        let response = self
            .rpc_client
            .abci_query(Some(path.to_string()), query_data, None, false)
            .await?;

        let abci_response = AbciQueryResponse::decode(response.value.as_slice())?;
        let result: T = cosmwasm_std::from_json(&abci_response.value)?;
        Ok(result)
    }

    /// Fetches account information for a given account ID.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The AccountId to fetch information for
    ///
    /// # Returns
    ///
    /// A Result containing an AccountInfoResponse or an error
    async fn fetch_account_info(
        &self,
        account_id: &AccountId,
    ) -> Result<AccountInfoResponse, Box<dyn std::error::Error>> {
        let path = format!("/cosmos/auth/v1beta1/accounts/{}", account_id);
        let data = self
            .rpc_client
            .abci_query(Some(path), Vec::new(), None, false)
            .await?;

        let any = Any::decode(data.value.as_slice())?;
        let account = BaseAccount::decode(any.value.as_slice())?;

        Ok(AccountInfoResponse {
            account_number: account.account_number,
            sequence: account.sequence,
        })
    }
}
