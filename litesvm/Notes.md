### Notes: My Understanding

1. For showing logs at console -> `cargo test create-account -- --show-output`
2. We can use specific crate for specific purpose instead of `solana-sdk` 
3. For adding crate as a dependency, we can use `cargo add solana-address@3.0.0`
4. state, it can not be accessed by outside , but instruction can be accessed because of tests use anchor generated types, for tests