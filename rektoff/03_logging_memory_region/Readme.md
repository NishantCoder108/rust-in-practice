### As I Understand

1. For creating workspace, we need first to create `folder` and then create `filename`. Its name would be `Cargo.toml`.
2. Add inside below code

   ```toml

    [workspace]
     members = []
     resolver = "2"

     [workspace.package]
     version = "0.1.0"
     edition = "2024"
     authors = ["Nishant"]
     license = "MIT"
   ```

3. Go inside `folder` and run command `cargo new client` and then run `cargo new program --lib`.

   ```toml

   [workspace]
    members = ["client", "program"]
    resolver = "2"

   ```

4. Build : `cargo-build-sbf`
5. Deploy : `solana program deploy ./target/deploy/loggi.so`
6. Received `program_id` and `signature`. Program id will be use at client side for interacting.
