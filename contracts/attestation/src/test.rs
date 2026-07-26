#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};
use crate::{AttestationContract, AttestationContractClient};
use crate::errors::Error;

fn url(env: &Env, s: &str) -> String {
    String::from_str(env, s)
}

fn setup() -> (Env, Address, Address, BytesN<32>) {
    let env = Env::default();
    env.mock_all_auths();
    let operator = Address::generate(&env);
    let other = Address::generate(&env);
    let provider_id = BytesN::from_array(&env, &[0u8; 32]);
    (env, operator, other, provider_id)
}

fn register(env: &Env, client: &AttestationContractClient, operator: &Address, pid: &BytesN<32>) {
    client.register_provider(pid, operator, &url(env, "https://rpc.example.com"));
}

#[test]
fn test_register_provider_success() {
    let (env, operator, _, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);

    register(&env, &client, &operator, &provider_id);

    let info = client.get_provider_info(&provider_id).unwrap();
    assert_eq!(info.operator, operator);
    assert_eq!(info.endpoint_url, url(&env, "https://rpc.example.com"));
}

#[test]
fn test_register_provider_duplicate() {
    let (env, operator, _, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);

    register(&env, &client, &operator, &provider_id);
    let result = client.try_register_provider(&provider_id, &operator, &url(&env, "https://rpc.example.com"));
    assert_eq!(result, Err(Ok(Error::AlreadyRegistered)));
}

#[test]
fn test_register_provider_missing_auth() {
    let env = Env::default();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);
    let operator = Address::generate(&env);
    let provider_id = BytesN::from_array(&env, &[0u8; 32]);

    let result = client.try_register_provider(&provider_id, &operator, &url(&env, "https://rpc.example.com"));
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// submit_attestation tests
// ---------------------------------------------------------------------------

#[test]
fn test_submit_attestation_success() {
    let (env, operator, _, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);
    register(&env, &client, &operator, &provider_id);

    client.submit_attestation(&provider_id, &100, &200, &9500, &150);

    let history = client.get_provider_history(&provider_id);
    assert_eq!(history.len(), 1);
    let a = history.get(0).unwrap();
    assert_eq!(a.period_start, 100);
    assert_eq!(a.period_end, 200);
    assert_eq!(a.uptime_percent, 9500);
    assert_eq!(a.avg_latency_ms, 150);
}

#[test]
fn test_submit_attestation_wrong_operator() {
    let (env, operator, _other, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);
    register(&env, &client, &operator, &provider_id);

    env.set_auths(&[]);
    let result = client.try_submit_attestation(&provider_id, &100, &200, &9500, &150);
    assert!(result.is_err());
}

#[test]
fn test_submit_attestation_invalid_period() {
    let (env, operator, _, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);
    register(&env, &client, &operator, &provider_id);

    let result = client.try_submit_attestation(&provider_id, &200, &100, &9500, &150);
    assert_eq!(result, Err(Ok(Error::InvalidPeriod)));
}

#[test]
fn test_submit_attestation_invalid_uptime() {
    let (env, operator, _, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);
    register(&env, &client, &operator, &provider_id);

    let result = client.try_submit_attestation(&provider_id, &100, &200, &10001, &150);
    assert_eq!(result, Err(Ok(Error::InvalidUptimeValue)));
}

#[test]
fn test_submit_attestation_provider_not_found() {
    let (env, _operator, _, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);

    let result = client.try_submit_attestation(&provider_id, &100, &200, &9500, &150);
    assert_eq!(result, Err(Ok(Error::ProviderNotFound)));
}

#[test]
fn test_submit_attestation_multiple() {
    let (env, operator, _, provider_id) = setup();
    let contract_id = env.register(AttestationContract, ());
    let client = AttestationContractClient::new(&env, &contract_id);
    register(&env, &client, &operator, &provider_id);

    client.submit_attestation(&provider_id, &100, &200, &9500, &150);
    client.submit_attestation(&provider_id, &200, &300, &9800, &120);

    let history = client.get_provider_history(&provider_id);
    assert_eq!(history.len(), 2);
    assert_eq!(history.get(0).unwrap().uptime_percent, 9500);
    assert_eq!(history.get(1).unwrap().uptime_percent, 9800);
}
