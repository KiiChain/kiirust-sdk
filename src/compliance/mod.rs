use request::ComplianceModuleRequest;

use crate::RwaClient;

pub mod request;

impl RwaClient {
    pub async fn add_compliance_module(&self, request: ComplianceModuleRequest) {}
    pub async fn remove_compliance_module(&self, request: ComplianceModuleRequest) {}
    pub async fn update_compliance_module(&self, request: ComplianceModuleRequest, active: bool) {}
}
