#![no_std]

mod types;
mod errors;
mod test;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String, Vec};
use crate::types::{Attestation, DataKey, ProviderInfo};
use crate::errors::Error;

#[contract]
pub struct AttestationContract;

#[contractimpl]
impl AttestationContract {
    /// Register a new provider. Only the operator address can register.
    /// Submitted data is self-reported and not verified by this contract.
    pub fn register_provider(
        env: Env,
        provider_id: BytesN<32>,
        operator: Address,
        endpoint_url: String,
    ) -> Result<(), Error> {
        operator.require_auth();
        if env.storage().persistent().has(&DataKey::Provider(provider_id.clone())) {
            return Err(Error::AlreadyRegistered);
        }
        let info = ProviderInfo {
            operator: operator.clone(),
            endpoint_url,
            registered_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&DataKey::Provider(provider_id.clone()), &info);
        env.storage().persistent().set(&DataKey::History(provider_id), &Vec::<Attestation>::new(&env));
        env.events().publish(
            ("ProviderRegistered",),
            (info.operator, info.registered_at),
        );
        Ok(())
    }

    /// Submit a signed attestation for a provider's uptime and latency over a period.
    ///
    /// The attestation values are self-reported by the registered operator.
    /// This contract does not verify the accuracy of the submitted metrics.
    /// Any dApp consuming this data should treat it as the provider's own claim.
    pub fn submit_attestation(
        env: Env,
        provider_id: BytesN<32>,
        period_start: u64,
        period_end: u64,
        uptime_percent: u32,
        avg_latency_ms: u32,
    ) -> Result<(), Error> {
        let info = env.storage().persistent()
            .get::<DataKey, ProviderInfo>(&DataKey::Provider(provider_id.clone()))
            .ok_or(Error::ProviderNotFound)?;
        info.operator.require_auth();
        if period_end <= period_start {
            return Err(Error::InvalidPeriod);
        }
        if uptime_percent > 10000 {
            return Err(Error::InvalidUptimeValue);
        }
        let mut history: Vec<Attestation> = env.storage().persistent()
            .get(&DataKey::History(provider_id.clone()))
            .unwrap_or(Vec::new(&env));
        let attestation = Attestation {
            period_start,
            period_end,
            uptime_percent,
            avg_latency_ms,
            submitted_at: env.ledger().timestamp(),
        };
        history.push_back(attestation);
        env.storage().persistent().set(&DataKey::History(provider_id), &history);
        env.events().publish(
            ("AttestationSubmitted",),
            (period_start, period_end, uptime_percent, avg_latency_ms),
        );
        Ok(())
    }

    pub fn get_provider_info(env: Env, provider_id: BytesN<32>) -> Option<ProviderInfo> {
        env.storage().persistent().get(&DataKey::Provider(provider_id))
    }

    /// Returns the full attestation history for a provider.
    /// All values in this history are self-reported by the provider's operator.
    /// This data is not verified by the contract.
    pub fn get_provider_history(env: Env, provider_id: BytesN<32>) -> Vec<Attestation> {
        env.storage().persistent()
            .get(&DataKey::History(provider_id))
            .unwrap_or(Vec::new(&env))
    }
}
