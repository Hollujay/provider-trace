# provider-trace

On-chain attestation system for Soroban RPC and indexer operators to post
signed, self-reported uptime and latency records.

## What this is and what it is not

provider-trace lets an RPC/indexer operator register a provider identity and
periodically submit signed uptime/latency attestations, pulled from their own
real metrics. Any dApp can query a provider's full history before deciding to
depend on them.

**This is self-attested data.** The contract's only trust boundary is
`require_auth()` confirming the submitter is the registered operator. The
contract does not verify the submitted numbers are true. It does not detect
lying. It does not solve Sybil resistance.

If you find yourself thinking "this proves a provider is reliable", stop and
re-read: this shows what the provider has reported about itself. Nothing more.

## Contract

### Types

```rust
struct ProviderInfo {
    operator: Address,
    endpoint_url: String,
    registered_at: u64,
}

struct Attestation {
    period_start: u64,
    period_end: u64,
    uptime_percent: u32,   // basis points, 0 to 10000
    avg_latency_ms: u32,
    submitted_at: u64,
}
```

### Functions

| Function | Auth | Description |
|---|---|---|
| `register_provider` | `operator.require_auth()` | Register a provider identity |
| `submit_attestation` | `ProviderInfo.operator.require_auth()` | Submit a self-reported attestation |
| `get_provider_info` | none | Read provider registration |
| `get_provider_history` | none | Read all attestations for a provider |

## CLI

```
cargo run -p provider-trace-cli -- fetch-metrics \
  --metrics-url https://your-node.example.com/metrics \
  --provider-id <32-byte-hex> \
  --period-start <unix-ts> \
  --period-end <unix-ts>
```

To test with a local metrics file:

```
cargo run -p provider-trace-cli -- fetch-metrics \
  --file examples/local-metrics-sample/metrics.txt \
  --provider-id 0000...0000 \
  --period-start 1000000 \
  --period-end 1003600
```

## Build & Test

```sh
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

Minimum Rust version: 1.79 (the SDK dependency may require a later version).

## Project status

Experimental. Do not use in production.

## License

TBD
