use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Error {
    AlreadyRegistered = 1,
    ProviderNotFound = 2,
    InvalidPeriod = 3,
    InvalidUptimeValue = 4,
}
