use std::str::FromStr;

use cosmrs::{
    proto::{
        cosmos::auth::v1beta1::BaseAccount, cosmwasm::wasm::v1::MsgExecuteContract, prost::Message,
    },
    rpc::{Client, HttpClient},
    tendermint::chain::Id,
    tx::{self, Fee, MessageExt, SignDoc, SignerInfo},
    AccountId, Any, Coin,
};

pub struct TokenContract {
    rpc_client: HttpClient,
    contract_address: String,
    chain_id: String,
}

struct AccountInfo {
    account_number: u64,
    sequence: u64,
}

impl TokenContract {
    pub fn new(rpc_client: HttpClient, contract_address: String, chain_id: String) -> Self {
        Self {
            rpc_client,
            contract_address,
            chain_id,
        }
    }

    async fn fetch_account_info(
        &self,
        account_id: &AccountId,
    ) -> Result<AccountInfo, Box<dyn std::error::Error>> {
        let path = format!("/cosmos/auth/v1beta1/accounts/{}", account_id);
        let data = self
            .rpc_client
            .abci_query(Some(path), Vec::new(), None, false)
            .await?;

        let any = Any::decode(data.value.as_slice())?;
        let account = BaseAccount::decode(any.value.as_slice())?;

        Ok(AccountInfo {
            account_number: account.account_number,
            sequence: account.sequence,
        })
    }

    pub async fn execute_contract<T: serde::Serialize>(
        &self,
        from: &str,
        msg: &T,
        funds: Vec<Coin>,
        signer: &cosmrs::crypto::secp256k1::SigningKey,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let execute_msg = MsgExecuteContract {
            sender: from.to_string(),
            contract: self.contract_address.clone(),
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
