use cosmrs::{crypto::secp256k1::SigningKey, Gas};

pub struct ComplianceModuleRequest {
    pub from: String,
    pub module_addr: String,
    pub signer: SigningKey,
    pub gas_limit: Gas,
}
