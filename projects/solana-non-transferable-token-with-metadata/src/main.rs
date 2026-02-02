use anyhow::Result;
use dotenvy::dotenv;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_associated_token_account_interface::instruction::create_associated_token_account;
use spl_token_2022_interface::{
    ID as TOKEN_2022_PROGRAM_ID,
    extension::{
        BaseStateWithExtensions, ExtensionType, StateWithExtensions,
        metadata_pointer::{
            MetadataPointer, instruction::initialize as initialize_metadata_pointer,
        },
    },
    instruction::{initialize_mint, initialize_non_transferable_mint},
    state::Mint,
};
use spl_token_metadata_interface::{
    instruction::{initialize as initialize_token_metadata, update_field},
    state::{Field, TokenMetadata},
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let fee_payer = keypair_from_env();

    // let fee_payer = payer_keypair.pubkey();

    println!("payer = {}", fee_payer.pubkey().to_string());
    // Create connection to local validator
    let client = RpcClient::new_with_commitment(
        String::from("http://api.devnet.solana.com"),
        CommitmentConfig::confirmed(),
    );
    let latest_blockhash = client.get_latest_blockhash().await?;

    // Generate a new keypair for the fee payer
    // let fee_payer = Keypair::new();

    // Airdrop 5 SOL to fee payer
    // let airdrop_signature = client
    //     .request_airdrop(&fee_payer.pubkey(), 2_000_000_000)
    //     .await?;

    // loop {
    //     let confirmed = client.confirm_transaction(&airdrop_signature).await?;
    //     if confirmed {
    //         break;
    //     }
    // }

    // Generate keypair for the mint
    let mint = Keypair::new();

    // Define Token metadata
    let token_metadata  = TokenMetadata {
        update_authority: Some(fee_payer.pubkey()).try_into()?,
        mint: mint.pubkey(),
        name: "FirstDayAtArena".to_string(),
        symbol : "FDAA".to_string(),
        uri : "https://raw.githubusercontent.com/NishantCoder108/rust-in-practice/refs/heads/main/projects/images/metadata.json".to_string(),
        additional_metadata: vec![("description".to_string(),"FirstDayAtArena represents builders who fight complexity with code and ship value on Solana. It symbolizes speed, decentralization, and a developer first mindset built only where performance matters most.".to_string())]
    };

    // Calculate space for mint with metadata pointer and token metadata extensions
    let mint_space = ExtensionType::try_calculate_account_len::<Mint>(&[
        ExtensionType::MetadataPointer,
        ExtensionType::NonTransferable,
    ])?;

    let metadata_len = token_metadata.tlv_size_of()?;

    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(mint_space + metadata_len)
        .await?;

    // Instruction to create new account for mint (token22)
    let create_mint_account_instruction = create_account(
        &fee_payer.pubkey(),    // payer
        &mint.pubkey(),         // new account (mint)
        mint_rent,              // lamports
        mint_space as u64,      // space
        &TOKEN_2022_PROGRAM_ID, // program id
    );

    let initialize_non_transferable_instruction =
        initialize_non_transferable_mint(&TOKEN_2022_PROGRAM_ID, &mint.pubkey())?;

    // Instruction to initialize metadata pointer (pointing to itself for self-managed metadata)
    let initialize_metadata_pointer_instruction = initialize_metadata_pointer(
        &TOKEN_2022_PROGRAM_ID,
        &mint.pubkey(),
        Some(fee_payer.pubkey()), // authority
        Some(mint.pubkey()),      // metadata address (pointing to self)
    )?;

    // Instruction to initialize mint account data
    let initialize_mint_instruction = initialize_mint(
        &TOKEN_2022_PROGRAM_ID,    // program id
        &mint.pubkey(),            // mint
        &fee_payer.pubkey(),       // mint authority
        Some(&fee_payer.pubkey()), // freeze authority
        0,                         // decimals
    )?;

    // Instruction to initialize token metadata
    let initialize_metadata_instruction = initialize_token_metadata(
        &TOKEN_2022_PROGRAM_ID,            // program id
        &mint.pubkey(),                    //metadata
        &fee_payer.pubkey(),               // update authority
        &mint.pubkey(),                    // mint
        &fee_payer.pubkey(),               // mint authority
        token_metadata.name.to_string(),   // name
        token_metadata.symbol.to_string(), // symbol
        token_metadata.uri.to_string(),    // uri
    );

    // Create update field instructions from token_metadata.additional_metadata
    // Additional metadata must be initialized separately using the update_field instruction
    // If the field already exists, it will be updated instead of creating a new field
    let update_field_instructions: Vec<_> = token_metadata
        .additional_metadata
        .iter()
        .map(|(key, value)| {
            update_field(
                &TOKEN_2022_PROGRAM_ID,
                &mint.pubkey(),
                &fee_payer.pubkey(),
                Field::Key(key.clone()),
                value.clone(),
            )
        })
        .collect();

    // Instruction to create associated token account
    let create_ata_instruction = create_associated_token_account(
        &fee_payer.pubkey(),    // payer
        &fee_payer.pubkey(),    // owner
        &mint.pubkey(),         // mint
        &TOKEN_2022_PROGRAM_ID, // token program
    );

    // Construct transaction with all instructions
    let mut instructions = vec![
        create_mint_account_instruction,
        initialize_non_transferable_instruction,
        initialize_metadata_pointer_instruction,
        initialize_mint_instruction,
        initialize_metadata_instruction,
        create_ata_instruction,
    ];
    instructions.extend(update_field_instructions);

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        latest_blockhash,
    );

    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    println!("Mint Address: {}", mint.pubkey());
    println!("Transaction Signature: {}", transaction_signature);

    // Fetch mint account
    let mint_account = client.get_account(&mint.pubkey()).await?;
    // Deserialize the mint account with extensions
    let mint_state = StateWithExtensions::<Mint>::unpack(&mint_account.data)?;

    // Get all extension types enabled on this mint
    let extension_types = mint_state.get_extension_types()?;
    println!("\nExtensions enabled: {:?}", extension_types);

    // Deserialize the MetadataPointer extension data
    let metadata_pointer = mint_state.get_extension::<MetadataPointer>()?;
    println!("\n{:#?}", metadata_pointer);

    // Deserialize the TokenMetadata extension data (variable-length)
    let token_metadata = mint_state.get_variable_len_extension::<TokenMetadata>()?;
    println!("\n{:#?}", token_metadata);

    Ok(())
}

fn keypair_from_env() -> Keypair {
    // let key_str = env::var("PAYER_PRIVATE_KEY").expect("PRIVATE_KEY not set");

    // let bytes: Vec<u8> = serde_json::from_str(&key_str).expect("Invalid private key format");

    // Keypair::from_bytes(&bytes).expect("Invalid keypair bytes")

    let key_str = env::var("PAYER_PRIVATE_KEY").expect("PRIVATE_KEY not set");

    // let bytes = bs58::decode(key_str)
    //     .into_vec()
    //     .expect("Invalid base58 key");

    Keypair::from_base58_string(&key_str)
}

/*
****---- Token logs -----*****


payer = HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs
Mint Address: 7GGpp37MAZWLf2yBrY5ShQknYNvjrHHU6LZcdFk9pWdH
Transaction Signature: sYzpSLR2DubpV1Shbq9Je96gdRSZERy7gxiuq8P9Q5rWzSiBtV3KcJTERPcfQUVgWjzqqYuJ59rRWKZMCCmi1Q5

Extensions enabled: [NonTransferable, MetadataPointer, TokenMetadata]

MetadataPointer {
    authority: OptionalNonZeroPubkey(
        HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs,
    ),
    metadata_address: OptionalNonZeroPubkey(
        7GGpp37MAZWLf2yBrY5ShQknYNvjrHHU6LZcdFk9pWdH,
    ),
}

TokenMetadata {
    update_authority: OptionalNonZeroPubkey(
        HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs,
    ),
    mint: 7GGpp37MAZWLf2yBrY5ShQknYNvjrHHU6LZcdFk9pWdH,
    name: "WarriorOfSol",
    symbol: "WOS",
    uri: "https://raw.githubusercontent.com/NishantCoder108/rust-in-practice/refs/heads/main/projects/images/metadata.json",
    additional_metadata: [
        (
            "description",
            "WarriorOfSOL represents builders who fight complexity with code and ship value on Solana. It symbolizes speed, decentralization, and a developer first mindset built only where performance matters most.",
        ),
    ],
}



URL : https://explorer.solana.com/address/7GGpp37MAZWLf2yBrY5ShQknYNvjrHHU6LZcdFk9pWdH/metadata?cluster=devnet&customUrl=http%3A%2F%2Flocalhost%3A8899


****---Other deploy non transferable token with metadata ----****


payer = HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs
Mint Address: 98f3PsJWBfgbA9pbTxfVyjiSfNYancM7yutHqkkwdQLG
Transaction Signature: 52aqH9z6dhxhqCpyGKPNJ6inAXhGrHxLSUecqeHH9j3tjvzWUQ1p4ML2La9HNBUsG6QDBMkCgUa3Z7cHrHStui8y

Extensions enabled: [NonTransferable, MetadataPointer, TokenMetadata]

MetadataPointer {
    authority: OptionalNonZeroPubkey(
        HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs,
    ),
    metadata_address: OptionalNonZeroPubkey(
        98f3PsJWBfgbA9pbTxfVyjiSfNYancM7yutHqkkwdQLG,
    ),
}

TokenMetadata {
    update_authority: OptionalNonZeroPubkey(
        HiMmuCbieNgDNFd9GbcbVSHYPGPuEgZWwQxJULaJVoVs,
    ),
    mint: 98f3PsJWBfgbA9pbTxfVyjiSfNYancM7yutHqkkwdQLG,
    name: "FirstDayAtArena",
    symbol: "FDAA",
    uri: "https://raw.githubusercontent.com/NishantCoder108/rust-in-practice/refs/heads/main/projects/images/metadata.json",
    additional_metadata: [
        (
            "description",
            "FirstDayAtArena represents builders who fight complexity with code and ship value on Solana. It symbolizes speed, decentralization, and a developer first mindset built only where performance matters most.",
        ),
    ],
}



*/
