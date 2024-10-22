//! Identity operations for the RWA SDK.
//!
//! This module provides functionality for registering, updating
//! and removing identities on the chain.

use cosmwasm_std::{Binary, Uint128};
use request::{
    AddClaimRequest, AddIdentityRequest, CheckUserForTokenComplianceRequest,
    GetValidatedClaimsRequest, RemoveClaimRequest, RemoveIdentityRequest, UpdateIdentityRequest,
};
use serde::{Deserialize, Serialize};

use crate::{ExecuteResponse, RwaClient};

pub mod request;

impl RwaClient {
    /// Adds a new identity to the chain.
    ///
    /// This function registers a new identity with the specified country.
    ///
    /// # Arguments
    ///
    /// * `request` - An `AddIdentityRequest` containing:
    ///   - `from`: The address initiating the transaction
    ///   - `country`: The country associated with the identity
    ///   - `signer`: The signing key for the transaction
    ///
    /// # Returns
    ///
    /// A `ExecuteResponse` containing information about the transaction if successful,
    /// or an error if the operation fails.
    pub async fn add_identity(
        &self,
        request: AddIdentityRequest,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::AddIdentity {
            country: request.country,
        };
        self.execute(
            &request.from,
            &msg,
            self.identity_address.clone(),
            vec![],
            &request.signer,
            request.gas_limit,
        )
        .await
    }

    /// Updates an existing identity on the chain.
    ///
    /// This function updates the country associated with an existing identity.
    ///
    /// # Arguments
    ///
    /// * `request` - An `UpdateIdentityRequest` containing:
    ///   - `from`: The address initiating the transaction
    ///   - `new_country`: The new country to associate with the identity
    ///   - `identity_owner`: The owner of the identity to update
    ///   - `signer`: The signing key for the transaction
    ///
    /// # Returns
    ///
    /// A `ExecuteResponse` containing information about the transaction if successful,
    /// or an error if the operation fails.
    pub async fn update_identity(
        &self,
        request: UpdateIdentityRequest,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::UpdateCountry {
            new_country: request.new_country,
            identity_owner: request.identity_owner,
        };
        self.execute(
            &request.from,
            &msg,
            self.identity_address.clone(),
            vec![],
            &request.signer,
            request.gas_limit,
        )
        .await
    }

    /// Removes an identity from the chain.
    ///
    /// This function removes an existing identity from the blockchain.
    ///
    /// # Arguments
    ///
    /// * `request` - A `RemoveIdentityRequest` containing:
    ///   - `from`: The address initiating the transaction
    ///   - `identity_owner`: The owner of the identity to remove
    ///   - `signer`: The signing key for the transaction
    ///
    /// # Returns
    ///
    /// A `ExecuteResponse` containing information about the transaction if successful,
    /// or an error if the operation fails.
    pub async fn remove_identity(
        &self,
        request: RemoveIdentityRequest,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::RemoveIdentity {
            identity_owner: request.identity_owner,
        };
        self.execute(
            &request.from,
            &msg,
            self.identity_address.clone(),
            vec![],
            &request.signer,
            request.gas_limit,
        )
        .await
    }

    /// Trusted issuer can add a claim to an identity on the chain.
    ///
    /// This function adds a new claim to an existing identity.
    ///
    /// # Arguments
    ///
    /// * `request` - An `AddClaimRequest` containing:
    ///   - `from`: The address of a trusted issuer initiating the transaction
    ///   - `claim`: The claim to add
    ///   - `identity_owner`: The owner of the identity to add the claim to
    ///   - `signer`: The signing key for the transaction
    ///
    /// # Returns
    ///
    /// A `ExecuteResponse` containing information about the transaction if successful,
    /// or an error if the operation fails.
    pub async fn add_claim(
        &self,
        request: AddClaimRequest,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::AddClaim {
            claim: request.claim,
            identity_owner: request.identity_owner,
        };
        self.execute(
            &request.from,
            &msg,
            self.identity_address.clone(),
            vec![],
            &request.signer,
            request.gas_limit,
        )
        .await
    }

    /// Trusted issuer can remove a claim from an identity on the chain.
    ///
    /// This function removes an existing claim from an identity.
    ///
    /// # Arguments
    ///
    /// * `request` - A `RemoveClaimRequest` containing:
    ///   - `from`: The address of a trusted issuer initiating the transaction
    ///   - `claim_topic`: The topic of the claim to remove
    ///   - `identity_owner`: The owner of the identity to remove the claim from
    ///   - `signer`: The signing key for the transaction
    ///
    /// # Returns
    ///
    /// A `ExecuteResponse` containing information about the transaction if successful,
    /// or an error if the operation fails.
    pub async fn remove_claim(
        &self,
        request: RemoveClaimRequest,
    ) -> Result<ExecuteResponse, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::RemoveClaim {
            claim_topic: request.claim_topic,
            identity_owner: request.identity_owner,
        };
        self.execute(
            &request.from,
            &msg,
            self.identity_address.clone(),
            vec![],
            &request.signer,
            request.gas_limit,
        )
        .await
    }

    /// Retrieves validated claims for a given identity.
    ///
    /// This function queries the blockchain for validated claims associated with a specific identity.
    ///
    /// # Arguments
    ///
    /// * `request` - A `GetValidatedClaimsRequest` containing:
    ///   - `identity_owner`: The owner of the identity to get claims for
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Claim`s if successful,
    /// or an error if the operation fails.
    pub async fn get_validated_claims(
        &self,
        request: GetValidatedClaimsRequest,
    ) -> Result<Vec<Claim>, Box<dyn std::error::Error>> {
        let msg = QueryMsg::GetValidatedClaimsForUser {
            identity_owner: request.identity_owner,
        };
        self.query(&self.identity_address, &msg).await
    }

    /// Checks token compliance for a user.
    ///
    /// This function queries the compliance contract to check if a user is compliant
    /// for a specific token operation.
    ///
    /// # Arguments
    ///
    /// * `request` - A `CheckUserForTokenComplianceRequest` containing:
    ///   - `token_address`: The address of the token contract
    ///   - `from`: The address of the user to check compliance for
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating compliance status if successful,
    /// or an error if the operation fails.
    pub async fn check_token_compliance(
        &self,
        request: CheckUserForTokenComplianceRequest,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let msg = QueryMsg::CheckTokenCompliance {
            token_address: request.token_address,
            from: Some(request.from),
            to: None,
            amount: None,
        };
        self.query(&self.compliance_address, &msg).await
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Claim {
    pub topic: Uint128,
    pub issuer: String,
    pub data: Binary,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum ExecuteMsg {
    AddIdentity {
        country: String,
    },
    RemoveIdentity {
        identity_owner: String,
    },
    UpdateCountry {
        new_country: String,
        identity_owner: String,
    },
    AddClaim {
        claim: Claim,
        identity_owner: String,
    },
    RemoveClaim {
        claim_topic: Uint128,
        identity_owner: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum QueryMsg {
    GetValidatedClaimsForUser {
        identity_owner: String,
    },
    GetIdentity {
        identity_owner: String,
    },
    CheckTokenCompliance {
        token_address: String,
        from: Option<String>,
        to: Option<String>,
        amount: Option<Uint128>,
    },
}
