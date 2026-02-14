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

---

### Resources:

https://github.com/dkcumming/solana-contact-memory/commit/0c4abec5447dafce5cc176b41b3dfa4a7155f4cd#diff-569c4866ec743e47bcc4258ec42720e55d00c382c8bc859b2916a4fe4f2ae030

**How string store in memory**:
https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=b4bd05b0ca30466d4f6b20732691c1cf
