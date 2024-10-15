use cosmrs::rpc::HttpClient;

pub struct ComplianceContract {
    rpc_client: HttpClient,
    address: String,
    chain_id: String,
}

impl ComplianceContract {}
