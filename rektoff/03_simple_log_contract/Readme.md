### As I understand:

1. For generating `.so` file, we need to use `crate-type = ["cdylib"]`.
2. For optimization :

   ```bash
   [profile.release] #It used for optimization
   lto = true # link time optimization
   opt-level = "s" #size optimization
   codegen-units = 1 #compile everything in one unit
   ```

3. For build the program, use command `cargo-build-sbf` .
4. For deploy the program:

   ```bash
    solana program deploy target/deploy/simple_log_contract.so
   ```

   - After deploy, we received `program_id` so we can interact from client

     ```text
     #like this:
     Program Id: AT5AQJhBnGwqAxLPr11WRE3javEPQZbGdAmK1jn9E1zs

     Signature: 2s7xZyq3qoHM7WrJaVSgLggL158qvaT5VBu4VNKrpCmrMuWPznPQscs6YHmS4EKvHLBPFkkxSihZoLTHvbaP18Y3
     ```
