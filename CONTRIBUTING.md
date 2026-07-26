# Contributing

## Scope

This project is intentionally scoped to self-attested data. Contributions that
introduce third-party verification, dispute mechanisms, or watcher/oracle
networks are out of scope and will not be accepted.

## Pull requests

- One logical change per commit.
- Every commit must compile and pass tests.
- Run `cargo clippy --workspace -- -D warnings` before pushing.
- Use conventional commit messages: `feat(contract):`, `fix(cli):`, etc.

## Testing

```sh
cargo test --workspace
```

Add tests for every new function. Negative tests (wrong auth, invalid input)
are as important as success tests.

## Code style

- No `unwrap()` outside `#[cfg(test)]` blocks.
- No floating point. Uptime values are basis points (`u32`, 0-10000).
- `require_auth()` is never optional.
- All public contract functions have doc comments restating that submitted
  data is self-reported and unverified.
- `snake_case` functions, `PascalCase` types.

## Out of scope

The following will not be added to this project:

- Third-party verification or attestation of provider data
- Dispute or challenge mechanisms
- Watcher or oracle networks
- Reputation scores computed from submitted data
- Any mechanism where a party other than the provider's registered operator
  can submit data about that provider
