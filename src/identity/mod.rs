//! Identity operations for the RWA SDK.
//!
//! This module provides functionality for registering, updating
//! and removing identities on the chain.

use cosmrs::{
    proto::{cosmos::auth::v1beta1::BaseAccount, prost::Message},
    rpc::Client,
    AccountId, Any,
};

use crate::RwaClient;

pub mod request;

impl RwaClient {
    /// Fetches account information for a given account ID.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The AccountId to fetch information for
    ///
    /// # Returns
    ///
    /// A Result containing an AccountInfoResponse or an error
    pub async fn fetch_account_info(
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
    pub async fn register_identity(&self) {}
    pub async fn update_identity(&self) {}
    pub async fn remove_identity(&self) {}
    pub async fn get_validated_claims(&self) {}
}

pub struct AccountInfoResponse {
    pub account_number: u64,
    pub sequence: u64,
}
