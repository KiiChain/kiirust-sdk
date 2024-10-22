//! Token operations for the RWA SDK.
//!
//! This module provides functionality for token transfers, balance checks,
//! and other token-related operations.

pub mod request;
use request::{TokenInfoRequest, TransferMessageRequest};

use crate::{ExecuteResponse, RwaClient};

impl RwaClient {
    /// Transfers tokens from the sender to a recipient.
    ///
    /// # Arguments
    ///
    /// * `request` - A TransferMessageRequest containing transfer details
    ///
    /// # Returns
    ///
    /// A `ExecuteResponse` containing information about the transaction if successful,
    /// or an error if the operation fails.
    pub async fn transfer(
        &self,
        request: TransferMessageRequest,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let msg = cw20::Cw20ExecuteMsg::Transfer {
            recipient: request.to.clone(),
            amount: request.amount.into(),
        };

        self.execute(
            &request.from,
            &msg,
            self.token_address.clone(),
            vec![],
            &request.signer,
            request.gas_limit,
        )
        .await
    }

    /// Transfers tokens from one address to another, given prior approval.
    ///
    /// # Arguments
    ///
    /// * `request` - A TransferMessageRequest containing transfer details
    ///
    /// # Returns
    ///
    /// A `ExecuteResponse` containing information about the transaction if successful,
    /// or an error if the operation fails.
    pub async fn transfer_from(
        &self,
        request: TransferMessageRequest,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let msg = cw20::Cw20ExecuteMsg::TransferFrom {
            owner: request.from.clone(),
            recipient: request.to.clone(),
            amount: request.amount.into(),
        };

        self.execute(
            &request.from,
            &msg,
            self.token_address.clone(),
            vec![],
            &request.signer,
            request.gas_limit,
        )
        .await
    }

    /// Retrieves information about the token.
    ///
    /// # Returns
    ///
    /// A Result containing a TokenInfoResponse or an error
    pub async fn coin_info(&self) -> Result<cw20::TokenInfoResponse, Box<dyn std::error::Error>> {
        let msg = cw20::Cw20QueryMsg::TokenInfo {};
        self.query(&self.token_address, &msg).await
    }

    /// Retrieves the token balance of a given address.
    ///
    /// # Arguments
    ///
    /// * `request` - A TokenInfoRequest containing the address to query
    ///
    /// # Returns
    ///
    /// A Result containing a BalanceResponse or an error
    pub async fn balance(
        &self,
        request: TokenInfoRequest,
    ) -> Result<cw20::BalanceResponse, Box<dyn std::error::Error>> {
        let msg = cw20::Cw20QueryMsg::Balance {
            address: request.address,
        };
        self.query(&self.token_address, &msg).await
    }
}
