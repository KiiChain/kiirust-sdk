use cosmrs::crypto::secp256k1::SigningKey;
use cosmwasm_std::Uint128;

use super::Claim;

/// Request structure for adding an identity
pub struct AddIdentityRequest {
    pub from: String,
    pub country: String,
    pub signer: SigningKey,
}
/// Request structure for updating an identity
pub struct UpdateIdentityRequest {
    pub from: String,
    pub new_country: String,
    pub identity_owner: String,
    pub signer: SigningKey,
}

/// Request structure for removing an identity
pub struct RemoveIdentityRequest {
    pub from: String,
    pub identity_owner: String,
    pub signer: SigningKey,
}

/// Request structure for adding a claim to user
pub struct AddClaimRequest {
    pub from: String,
    pub claim: Claim,
    pub identity_owner: String,
    pub signer: SigningKey,
}

/// Request structure for removing a claim
pub struct RemoveClaimRequest {
    pub from: String,
    pub claim_topic: Uint128,
    pub identity_owner: String,
    pub signer: SigningKey,
}

/// Request structure for retrieving validated claims for user
pub struct GetValidatedClaimsRequest {
    pub identity_owner: String,
}

/// Request structure for to check if user is compliant for token
pub struct CheckUserForTokenComplianceRequest {
    pub token_address: String,
    pub from: String,
}
