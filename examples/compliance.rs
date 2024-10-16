use erc3643sdk::{compliance::request::ComplianceModuleRequest, RwaClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = RwaClient::new(
        "rpc_url",
        "chain_id",
        "token_address",
        "identity_address",
        "compliance_address",
    )?;

    // add
    client
        .add_compliance_module(ComplianceModuleRequest {
            owner_addr: "owner".to_string(),
            module_addr: "module".to_string(),
        })
        .await;

    // remove
    client
        .remove_compliance_module(ComplianceModuleRequest {
            owner_addr: "owner".to_string(),
            module_addr: "module".to_string(),
        })
        .await;

    // update
    let request = ComplianceModuleRequest {
        owner_addr: "owner".to_string(),
        module_addr: "module".to_string(),
    };
    client.update_compliance_module(request, true).await;

    Ok(())
}
