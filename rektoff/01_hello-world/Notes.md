## Notes - Mental Modeling

#### 1. What is `#[program]`?
> This is an attribute macro in Anchor that designates the module containing all program instructions. Functions defined inside this module become program instructions, which can be called directly from the client side in a Solana program.

 <!-- #### 2.  -->


 #### Important Points:
 1. When we deploy programs on chain so it generate `keypair.json file`. this manage the program.
 2. For run local solana validator. we use `solana-test-validator`, it will act like onchain.
 3. For reset the validator, We can use `solana-test-validator --reset` command
 4. We can deploy on `localnet` and `devnet` and `mainnet` after build. Just change the url and anchor.toml file's localnet to devnet
 5. When we deploy program, IDL is also publish on onchain, no need to deploy on manually. [see here](http://explorer.solana.com/address/GCagCXvTjjcH4LrmPoCnVAypBq2yhnnU3McSF8sQFUqC/idl?cluster=devnet)
 6. `macros` : It's a piece of code that generate other piece of code , we can say metaprogramming
 7. For creating new project or adding other project inside one single workspace, we have same command `cargo new projectName`
 8. If we have already `folder` so we can use `cargo init` inside folder
 9. How to integrate with client side : [See Rust Client](https://www.anchor-lang.com/docs/clients/rust)
10. We write client side and test it at `locally` and `devnet`