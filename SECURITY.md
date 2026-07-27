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

Please distinguish between two categories of report:

1. **Trust model feedback.** If your issue concerns the self-attested
   nature of the data, the lack of on-chain verification, or the absence
   of a dispute mechanism — that is working as designed and is not a
   security bug. Please file it as a public GitHub Issue or Discussion.

2. **Genuine security vulnerabilities.** If you have found a bug that
   could bypass `require_auth()`, compromise the operator key check, or
   otherwise subvert the contract's intended security boundary, report it
   **privately** via the GitHub Security Advisories tab
   (https://github.com/anomalyco/provider-trace/security/advisories/new)
   rather than a public issue. Publishing a working exploit before a fix
   ships would allow it to be used against live deployments on mainnet.
