use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{EncodableKey, Keypair, Signer},
    transaction::Transaction,
};
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};
use std::str::FromStr;

// Only know this after deploying, replace with read data
const PROGRAM_PUBKEY: &str = "DTKiRRtmJeqBAPUP8KuGZFCNf2UzuAvKYTULdcatMgEM";

fn main() -> Result<()> {
    // Connect to local validator
    let rpc_url = "http://localhost:8899";
    let client = RpcClient::new(rpc_url.to_string());

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
        // data: vec![],
        data: vec![0],
    };

    // Create message and transaction
    let message = Message::new(&[instruction], Some(&client_keypair.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.sign(&[&client_keypair], recent_blockhash);

    println!("Invoking program: {}", program_id);

    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction succeeded! Signature: {}", signature);

    // Unpack and print the logs
    let tx_info = client.get_transaction(&signature, UiTransactionEncoding::Json)?;
    if let Some(meta) = tx_info.transaction.meta {
        if let OptionSerializer::Some(logs) = meta.log_messages {
            println!("Logs from {}:", signature);
            for log in logs {
                println!("  {}", log);
            }
        } else {
            println!("No logs");
        }
    }

    Ok(())
}

/*
****----Logs---****

Logs from 5pqhvcrJaBBTSbExrTPXtUSWLmfrm1GgcH4M6J8Jii2V7NEGoVWERzwkXJR3khzjZbktivA5a1JvxMDDB6qYghvS:
  Program DTKiRRtmJeqBAPUP8KuGZFCNf2UzuAvKYTULdcatMgEM invoke [1]
  Program log: Hello, world!
  Program log: This is a simple logging program for learning.
  Program log: Program executed successfully!
  Program log: instr args   = Pointer { addr: 0x400000010, metadata: 1 }
  Program log: vec heap     = 0x300007ff4
  Program log: vec stack    = 0x200004f88
  Program log: IN_RO static address = 0x100005490
  Program log: IN_RO static value   = 0x123456789abcdef
  Program log: process_instruction address: 0x100000540
  Program log: log_memory_regions address: 0x100000698
  Program DTKiRRtmJeqBAPUP8KuGZFCNf2UzuAvKYTULdcatMgEM consumed 5056 of 200000 compute units
  Program DTKiRRtmJeqBAPUP8KuGZFCNf2UzuAvKYTULdcatMgEM success
*/
