use anchor_client::{Client, Cluster, solana_sdk::commitment_config::CommitmentConfig};
use anchor_lang::{declare_program, prelude::system_program};
use solana_sdk::signature::{EncodableKey, Keypair, Signer};
use std::rc::Rc;

declare_program!(hello_world); //(reads IDL automatically)

use hello_world::{ID, accounts::NewAccount, client::accounts, client::args};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let payer = read_keypair_file("keypair.json")?;
    let keypair_path = "/Users/nishant/.config/solana/id.json";
    let client_keypair = Keypair::read_from_file(keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;

    let client = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(&client_keypair),
        CommitmentConfig::confirmed(),
    );

    let program = client.program(ID)?;

    let new_account = Keypair::new();
    let new_account_pubkey = &new_account.pubkey();

    println!("\nSend transaction with initialize and increment instructions");

    let initialize_ix = program
        .request()
        .accounts(accounts::Initialize {
            new_account: *new_account_pubkey,
            signer: client_keypair.pubkey(),
            system_program: system_program::ID,
        })
        .args(args::Initialize { data: 54 })
        .instructions()?
        .remove(0); //It will pop current instruction

    let signature = program
        .request()
        .instruction(initialize_ix)
        .signer(new_account)
        .send()
        .await?;
    println!("   Transaction confirmed: {}", signature);

    println!("\nFetch counter account data");
    let new_account_data: NewAccount = program.account::<NewAccount>(*new_account_pubkey).await?;
    println!("   New Account value: {}", new_account_data.data);

    Ok(())
}
