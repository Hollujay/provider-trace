# provider-trace

[![CI](https://github.com/Hollujay/provider-trace/actions/workflows/ci.yml/badge.svg)](https://github.com/Hollujay/provider-trace/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

A Soroban contract and CLI for RPC and indexer operators to post signed, self-reported uptime and latency records on-chain, so a dApp can check a provider's history before depending on it.

## What this is (and isn't)

This data is self-attested. It is not independently verified.

- Only a provider's registered operator can submit data about that provider. The contract enforces this with `require_auth()`, nothing more.

- The contract does not check whether submitted uptime or latency values are true.

- There is no dispute mechanism, no slashing, no third-party verification. These are explicitly out of scope, see CONTRIBUTING.md.

Treat this data the way you'd treat a provider's own marketing claim: useful as a structured, queryable history, not as proof of reliability.

## Quick start

Requires Rust (stable, edition 2021) and the Stellar CLI if you want to deploy the contract yourself.

```bash

git clone https://github.com/Hollujay/provider-trace.git

cd provider-trace

cargo build --workspace

cargo test --workspace

```

Parse a node's real metrics into attestation values:

```bash

cargo run --bin provider-trace-cli -- fetch-metrics \

  --file examples/local-metrics-sample/metrics.txt \

  --provider-id <32-byte-hex-id> \

  --period-start <unix-seconds> \

  --period-end <unix-seconds>

```

```

Parsed attestation values (self-reported, unverified):

  Provider ID:     ...

  Period start:    ...

  Period end:      ...

  Uptime (bp):     9975 (99.75%)

  Avg latency ms:  42

```

Or fetch directly from a live node with `--metrics-url` instead of `--file`.

## Architecture

```

Node's /metrics endpoint

        |

        v

   CLI fetch-metrics (parses uptime/latency, no floats, basis points)

        |

        v

   submit_attestation() -- operator.require_auth()

        |

        v

   Contract storage (per-provider history, append-only)

        |

        v

   get_provider_history() -- anyone can query, no auth needed

```

`register_provider` binds a provider ID to one operator address, permanently. `submit_attestation` is the only place the trust boundary lives, only that operator can post for that provider.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for scope, setup, and code style, including what this project explicitly will not add. Security issues, see [SECURITY.md](./SECURITY.md).

## Maintainer

[@Hollujay](https://github.com/Hollujay)

