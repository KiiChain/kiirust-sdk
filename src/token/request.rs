use cosmrs::crypto::secp256k1::SigningKey;

/// Request structure for token transfers
pub struct TransferMessageRequest {
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub signer: SigningKey,
}

/// Request structure for token info queries
pub struct TokenInfoRequest {
    pub address: String,
}
