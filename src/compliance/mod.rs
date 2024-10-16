//! Compliance operations for the RWA SDK.
//!
//! This module provides functionality for registering, updating
//! and removing compliance modules on the chain.

use request::ComplianceModuleRequest;
use serde::{Deserialize, Serialize};

use crate::RwaClient;

pub mod request;

impl RwaClient {
    /// Adds a new compliance module.
    ///
    /// # Arguments
    ///
    /// * `request` - A ComplianceModuleRequest containing module details
    pub async fn add_compliance_module(
        &self,
        module_name: &str,
        request: ComplianceModuleRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::AddComplianceModule {
            token_address: self.token_address.clone(),
            module_address: request.module_addr,
            module_name: module_name.to_string(),
        };

        self.execute(
            &request.from,
            &msg,
            self.compliance_address.clone(),
            vec![],
            &request.signer,
        )
        .await
    }

    /// Removes a compliance module.
    ///
    /// # Arguments
    ///
    /// * `request` - A ComplianceModuleRequest containing module details
    pub async fn remove_compliance_module(
        &self,
        request: ComplianceModuleRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::RemoveComplianceModule {
            token_address: self.token_address.clone(),
            module_address: request.module_addr,
        };

        self.execute(
            &request.from,
            &msg,
            self.compliance_address.clone(),
            vec![],
            &request.signer,
        )
        .await
    }

    /// Updates the status of a compliance module.
    ///
    /// # Arguments
    ///
    /// * `request` - A ComplianceModuleRequest containing module details
    /// * `active` - A boolean indicating whether the module should be active
    pub async fn update_compliance_module(
        &self,
        request: ComplianceModuleRequest,
        active: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let msg = ExecuteMsg::UpdateComplianceModule {
            token_address: self.token_address.clone(),
            module_address: request.module_addr,
            active,
        };

        self.execute(
            &request.from,
            &msg,
            self.compliance_address.clone(),
            vec![],
            &request.signer,
        )
        .await
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum ExecuteMsg {
    AddComplianceModule {
        token_address: String,
        module_address: String,
        module_name: String,
    },
    RemoveComplianceModule {
        token_address: String,
        module_address: String,
    },
    UpdateComplianceModule {
        token_address: String,
        module_address: String,
        active: bool,
    },
}
