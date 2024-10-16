//! Compliance operations for the RWA SDK.
//!
//! This module provides functionality for registering, updating
//! and removing compliance modules on the chain.

use request::ComplianceModuleRequest;

use crate::RwaClient;

pub mod request;

impl RwaClient {
    /// Adds a new compliance module.
    ///
    /// # Arguments
    ///
    /// * `request` - A ComplianceModuleRequest containing module details
    pub async fn add_compliance_module(&self, request: ComplianceModuleRequest) {
        todo!()
    }

    /// Removes a compliance module.
    ///
    /// # Arguments
    ///
    /// * `request` - A ComplianceModuleRequest containing module details
    pub async fn remove_compliance_module(&self, request: ComplianceModuleRequest) {
        todo!()
    }

    /// Updates the status of a compliance module.
    ///
    /// # Arguments
    ///
    /// * `request` - A ComplianceModuleRequest containing module details
    /// * `active` - A boolean indicating whether the module should be active
    pub async fn update_compliance_module(&self, request: ComplianceModuleRequest, active: bool) {
        todo!()
    }
}
