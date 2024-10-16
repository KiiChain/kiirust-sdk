use cosmrs::rpc::HttpClient;
use cosmrs::{
    proto::cosmwasm::wasm::v1::MsgExecuteContract,
    rpc::Client,
    tendermint::chain::Id,
    tx::{self, Fee, MessageExt, SignDoc, SignerInfo},
    AccountId, Coin,
};
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
}

impl RwaClient {
    pub fn new(
        rpc_url: &str,
        chain_id: &str,
        token_address: &str,
        identity_address: &str,
        compliance_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let rpc_client = HttpClient::new(rpc_url)?;

        Ok(Self {
            rpc_client,
            chain_id: chain_id.to_string(),
            token_address: token_address.to_string(),
            identity_address: identity_address.to_string(),
            compliance_address: compliance_address.to_string(),
        })
    }
    pub async fn execute<T: serde::Serialize>(
        &self,
        from: &str,
        msg: &T,
        contract_address: String,
        funds: Vec<Coin>,
        signer: &cosmrs::crypto::secp256k1::SigningKey,
    ) -> Result<String, Box<dyn std::error::Error>> {
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

        let amount = Coin {
            amount: 68u8.into(),
            denom: "kii".parse().unwrap(),
        };
        let gas = 500_000u64;
        let fee = Fee::from_amount_and_gas(amount, gas);

        // Prepare authentication info
        let auth_info = SignerInfo::single_direct(Some(signer.public_key()), account_info.sequence)
            .auth_info(fee);

        // Construct the sign doc
        let chain_id = Id::from_str(&self.chain_id)?;
        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_info.account_number)?;

        let tx_raw = sign_doc.sign(signer)?;

        let tx_bytes = tx_raw.to_bytes()?;

        let response = self.rpc_client.broadcast_tx_commit(tx_bytes).await?;

        Ok(response.hash.to_string())
    }
}
