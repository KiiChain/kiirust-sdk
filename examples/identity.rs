use cosmrs::crypto::secp256k1::SigningKey;
use cosmwasm_std::{Binary, Uint128};
use erc3643sdk::identity::request::{
    AddClaimRequest, AddIdentityRequest, CheckUserForTokenComplianceRequest,
    GetValidatedClaimsRequest, RemoveClaimRequest, RemoveIdentityRequest, UpdateIdentityRequest,
};
use erc3643sdk::identity::Claim;
use erc3643sdk::RwaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Mainnet — use testnet RPC / oro_1336-1 for Oro
    let client = RwaClient::new(
        "https://rpc.kiivalidator.com",
        "kiichain_1783-1",
        "kii1token...",
        "kii1identity...",
        "kii1compliance...",
        "akii",
        10,
    )?;

    // Add a new identity
    let add_identity_request = AddIdentityRequest {
        from: "kii1sender...".to_string(),
        country: "US".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let add_result = client.add_identity(add_identity_request).await?;
    println!("Add identity transaction hash: {}", add_result.tx_hash);

    // Update an identity
    let update_identity_request = UpdateIdentityRequest {
        from: "kii1sender...".to_string(),
        new_country: "CA".to_string(),
        identity_owner: "kii1owner...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let update_result = client.update_identity(update_identity_request).await?;
    println!(
        "Update identity transaction hash: {}",
        update_result.tx_hash
    );

    // Add a claim to an identity
    let add_claim_request = AddClaimRequest {
        from: "kii1issuer...".to_string(),
        claim: Claim {
            topic: Uint128::new(1),
            issuer: "kii1issuer...".to_string(),
            data: Binary::from(b"claim data"),
            uri: "https://example.com/claim".to_string(),
        },
        identity_owner: "kii1owner...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let add_claim_result = client.add_claim(add_claim_request).await?;
    println!("Add claim transaction hash: {}", add_claim_result.tx_hash);

    // Get validated claims for an identity
    let get_claims_request = GetValidatedClaimsRequest {
        identity_owner: "kii1owner...".to_string(),
    };
    let claims = client.get_validated_claims(get_claims_request).await?;
    println!("Validated claims: {:?}", claims);

    // Remove a claim from an identity
    let remove_claim_request = RemoveClaimRequest {
        from: "kii1issuer...".to_string(),
        claim_topic: Uint128::new(1),
        identity_owner: "kii1owner...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let remove_claim_result = client.remove_claim(remove_claim_request).await?;
    println!(
        "Remove claim transaction hash: {}",
        remove_claim_result.tx_hash
    );

    // Remove an identity
    let remove_identity_request = RemoveIdentityRequest {
        from: "kii1sender...".to_string(),
        identity_owner: "kii1owner...".to_string(),
        signer: SigningKey::from_slice(&[/* your private key */])?,
        gas_limit: 5000,
    };
    let remove_result = client.remove_identity(remove_identity_request).await?;
    println!(
        "Remove identity transaction hash: {}",
        remove_result.tx_hash
    );

    // Check token compliance for a user
    let compliance_request = CheckUserForTokenComplianceRequest {
        token_address: "kii1token...".to_string(),
        from: "kii1user...".to_string(),
    };
    let is_compliant = client.check_token_compliance(compliance_request).await?;
    println!("Is user compliant: {}", is_compliant);

    Ok(())
}
