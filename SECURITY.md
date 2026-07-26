# Security

## Trust model

provider-trace has a deliberately narrow trust boundary:

1. **Authentication:** Only `require_auth()` for the registered operator
   address. If an operator's key is compromised, an attacker can submit
   arbitrary attestations for that provider.
2. **No verification:** The contract does not verify that submitted
   attestation values are accurate. All data is self-reported.

## What this means

- A provider can submit false uptime or latency values. The contract will
   accept them and so will any dApp that reads them.
- There is no slashing, bonding, or dispute mechanism. Those are out of
   scope for this project.
- Consumers of this data should treat it as the provider's own marketing
   claim, not as an independently verified fact.

## Reporting issues

If you find a bug in the contract logic or CLI, open an issue on GitHub.
Do not open security issues about the trust model — it is working as
designed.
