#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, IntoVal, Vec};
use crate::{AttestationContract, AttestationContractClient};
use crate::errors::Error;
use crate::types::{Attestation, ProviderInfo};

fn setup() -> (Env, (Address, Address, Address), (BytesN<32>, BytesN<32>)) {
    let env = Env::default();
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let provider_id = BytesN::from_array(&env, &[0u8; 32]);
    let provider_id2 = BytesN::from_array(&env, &[1u8; 32]);
    (env, (user1, user2, user3), (provider_id, provider_id2))
}

#[test]
fn test_register_provider_success() {
    let (env, (user1, _, _), (provider_id, _)) = setup();
    let client = AttestationContractClient::new(&env, &env.register_contract(None, AttestationContract));

    client.register_provider(&provider_id, &user1, &"https://rpc.example.com".into_val(&env));

    let info = client.get_provider_info(&provider_id).unwrap();
    assert_eq!(info.operator, user1);
    assert_eq!(info.endpoint_url, "https://rpc.example.com".into_val::<String>(&env));
}

#[test]
fn test_register_provider_duplicate() {
    let (env, (user1, _, _), (provider_id, _)) = setup();
    let client = AttestationContractClient::new(&env, &env.register_contract(None, AttestationContract));

    client.register_provider(&provider_id, &user1, &"https://rpc.example.com".into_val(&env));
    let result = client.try_register_provider(&provider_id, &user1, &"https://rpc.example.com".into_val(&env));
    assert_eq!(result, Err(Ok(Error::AlreadyRegistered)));
}

#[test]
fn test_register_provider_missing_auth() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AttestationContract);
    let client = AttestationContractClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let provider_id = BytesN::from_array(&env, &[0u8; 32]);

    let result = client.try_register_provider(&provider_id, &user, &"https://rpc.example.com".into_val(&env));
    assert_eq!(result, Err(Ok(Error::AlreadyRegistered)));
}
