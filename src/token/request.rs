pub struct TransferMessageRequest {
    pub recipient_addr: String,
    pub amount: u64,
    pub owner_addr: Option<String>,
}

pub struct TokenInfoRequest {
    pub owner_addr: String,
}
