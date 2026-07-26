use soroban_sdk::{contracttype, Address, BytesN, String};

#[contracttype]
pub struct ProviderInfo {
    pub operator: Address,
    pub endpoint_url: String,
    pub registered_at: u64,
}

#[contracttype]
pub struct Attestation {
    pub period_start: u64,
    pub period_end: u64,
    pub uptime_percent: u32,
    pub avg_latency_ms: u32,
    pub submitted_at: u64,
}

#[contracttype]
pub enum DataKey {
    Provider(BytesN<32>),
    History(BytesN<32>),
}
