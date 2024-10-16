use cosmrs::rpc::{HttpClient, Url};

pub mod coin;
pub mod compliance;
pub mod identity;

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
}
