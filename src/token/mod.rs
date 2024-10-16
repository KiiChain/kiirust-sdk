// pub mod request;
pub mod request;
use request::{TokenInfoRequest, TransferMessageRequest};

use crate::RwaClient;

impl RwaClient {
    pub async fn transfer(
        &self,
        request: TransferMessageRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
        )
        .await
    }
    pub async fn transfer_from(
        &self,
        request: TransferMessageRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
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
        )
        .await
    }

    pub async fn coin_info(&self) -> Result<cw20::TokenInfoResponse, Box<dyn std::error::Error>> {
        let msg = cw20::Cw20QueryMsg::TokenInfo {};
        self.query(&self.token_address, &msg).await
    }

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
