use cosmrs::rpc::HttpClient;

pub struct IdentityContract {
    rpc_client: HttpClient,
    address: String,
    chain_id: String,
}

impl IdentityContract {}
