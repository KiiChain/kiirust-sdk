// pub mod request;
pub mod request;
use request::{TokenInfoRequest, TransferMessageRequest};

use crate::RwaClient;

impl RwaClient {
    pub async fn transfer(&self, request: TransferMessageRequest) {}
    pub async fn transfer_from(&self, request: TransferMessageRequest) {}
    pub async fn is_verified_transfer(&self) {}
    pub async fn coin_info(&self) {}
    pub async fn balance(&self, request: TokenInfoRequest) {}
}
