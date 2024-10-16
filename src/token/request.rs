use cosmrs::crypto::secp256k1::SigningKey;

pub struct TransferMessageRequest {
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub signer: SigningKey,
}

pub struct TokenInfoRequest {
    pub address: String,
}
