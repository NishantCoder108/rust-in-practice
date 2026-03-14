use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::CommitmentConfig;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{EncodableKey, Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

// Only know this after deploying, replace with read data
const PROGRAM_PUBKEY: &str = "AT5AQJhBnGwqAxLPr11WRE3javEPQZbGdAmK1jn9E1zs";

fn main() -> Result<()> {
    // Connect to local validator
    let rpc_url = "http://localhost:8899";
    // let client = RpcClient::new(rpc_url.to_string());
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"),
        CommitmentConfig::confirmed(),
    );

    let program_id = Pubkey::from_str(PROGRAM_PUBKEY)?;

    // Load keypair - best to use absolute path e.g. /home/<USER>/.config/solana/id.json
    let keypair_path = "/Users/nishant/.config/solana/id.json";
    let client_keypair = Keypair::read_from_file(keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;

    // Transactions require a recent blockhash
    let recent_blockhash = client.get_latest_blockhash()?;

    // Create instruction to invoke the program
    let instruction = Instruction {
        program_id,
        accounts: vec![],
        data: vec![],
    };

    // Create message and transaction
    let message = Message::new(&[instruction], Some(&client_keypair.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.sign(&[&client_keypair], recent_blockhash);

    println!("Invoking program: {}", program_id);

    // Send the transaction
    // let signature = client.send_and_confirm_transaction(&transaction)?;
    let sig = client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction succeeded! Signature: {:?}", sig);

    Ok(())
}
