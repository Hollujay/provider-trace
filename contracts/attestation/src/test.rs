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
