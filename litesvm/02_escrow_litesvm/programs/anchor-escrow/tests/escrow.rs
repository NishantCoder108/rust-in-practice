use {
    anchor_escrow::{accounts::Make as MakeAccounts, instruction::Make as MakeIx},
    anchor_lang::{
        prelude::msg, solana_program::example_mocks::solana_rpc_client::rpc_client::RpcClient,
        InstructionData, ToAccountMetas,
    },
    anchor_spl::associated_token::{self, get_associated_token_address},
    litesvm::LiteSVM,
    litesvm_token::{
        spl_token::ID as TOKEN_PROGRAM_ID, CreateAssociatedTokenAccount, CreateMint, MintTo,
    },
    solana_address::{address, Address},
    solana_instruction::{account_meta::AccountMeta, Instruction},
    solana_keypair::{read_keypair_file, Keypair},
    // solana_message::Instruction,
    solana_message::{Message, VersionedMessage},
    solana_native_token::LAMPORTS_PER_SOL,
    solana_pubkey::Pubkey,
    solana_sdk_ids::system_program::ID as SYSTEM_PROGRAM_ID,
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::{ops::Add, str::FromStr},
};
#[test]
fn escrow() {
    let mut svm = LiteSVM::new();
    // let payer = Keypair::new();

    // svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL)
    //     .expect("Failed to airdop sol");

    let program_keypair =
        read_keypair_file("../../target/deploy/anchor_escrow-keypair.json").unwrap();
    let program_id = program_keypair.pubkey();

    println!("Program Keypair : {:?}", program_keypair);

    let program_bytes = include_bytes!("../../../target/deploy/anchor_escrow.so");

    let _ = svm.add_program(program_id, program_bytes);

    // let program_id_pukey: Pubkey = program_id.into();
    let program_id_key = Pubkey::new_from_array(program_id.to_bytes());
    //load account from devnet
    // let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
    // let account_address =
    //     Address::from_str("DRYvf71cbF2s5wgaJQvAGkghMkRcp5arvsK2w97vXhi2").unwrap();

    // ============================================================================
    // Create and fund test accounts
    // ============================================================================
    let maker = Keypair::new();
    let taker = Keypair::new();
    svm.airdrop(&maker.pubkey(), 10_000_000_000).unwrap(); // 10 SOL
    svm.airdrop(&taker.pubkey(), 10_000_000_000).unwrap(); // 10 SOL

    // ============================================================================
    // Token Setup: Create mints and token accounts
    // Token swap flow: Maker offers mint_a tokens, wants mint_b tokens in return
    // ============================================================================

    // Create two token mints with maker as authority
    let mint_a = CreateMint::new(&mut svm, &maker)
        .authority(&maker.pubkey())
        .decimals(6)
        .send()
        .unwrap();

    let mint_b = CreateMint::new(&mut svm, &maker)
        .authority(&maker.pubkey())
        .decimals(6)
        .send()
        .unwrap();

    // Create all associated token accounts upfront for clarity
    // Maker's account for mint_a (will deposit into escrow)
    let maker_ata_a = CreateAssociatedTokenAccount::new(&mut svm, &maker, &mint_a)
        .owner(&maker.pubkey())
        .send()
        .unwrap();

    // Taker's account for mint_b (will send to maker)
    let taker_ata_b = CreateAssociatedTokenAccount::new(&mut svm, &taker, &mint_b)
        .owner(&taker.pubkey())
        .send()
        .unwrap();

    // Taker's account for mint_a (will receive from escrow)
    let taker_ata_a = CreateAssociatedTokenAccount::new(&mut svm, &taker, &mint_a)
        .owner(&taker.pubkey())
        .send()
        .unwrap();

    // Maker's account for mint_b (will receive from taker)
    let maker_ata_b = CreateAssociatedTokenAccount::new(&mut svm, &maker, &mint_b)
        .owner(&maker.pubkey())
        .send()
        .unwrap();

    let escrow = Pubkey::find_program_address(
        &[b"escrow", maker.pubkey().as_ref(), &123u64.to_le_bytes()],
        &program_id_key,
    )
    .0;
    msg!("Escrow PDA: {}\n", escrow);
    // Derive the PDA for the vault associated token account using the escrow PDA and Mint A
    let vault = get_associated_token_address(&escrow, &address_to_pubkey(mint_a));
    msg!("Vault PDA: {}\n", vault);

    // Define program IDs for associated token program, token program, and system program
    let asspciated_token_program = spl_associated_token_account::ID;
    let token_program = TOKEN_PROGRAM_ID;
    let system_program = SYSTEM_PROGRAM_ID;

    // Mint 1,000 tokens (with 6 decimal places) of Mint A to the maker's associated token account
    MintTo::new(&mut svm, &maker, &mint_a, &maker_ata_a, 1000000000)
        .send()
        .unwrap();

    // Create the "Make" instruction to deposit tokens into the escrow
    let make_ix = Instruction {
        // `Instruction::program_id` is an `Address` in this SDK
        program_id,
        accounts: MakeAccounts {
            maker: address_to_pubkey(maker.pubkey()),
            mint_a: address_to_pubkey(mint_a),
            mint_b: address_to_pubkey(mint_b),
            maker_ata_a: address_to_pubkey(maker_ata_a),
            escrow: escrow,
            vault: vault,
            associated_token_program: address_to_pubkey(asspciated_token_program),
            token_program: address_to_pubkey(token_program),
            system_program: system_program,
        }
        .to_account_metas(None)
        .into_iter()
        .map(|m| AccountMeta {
            pubkey: pubkey_to_address(m.pubkey),
            is_signer: m.is_signer,
            is_writable: m.is_writable,
        })
        .collect(),
        data: MakeIx {
            deposit: 10,
            seed: 123u64,
            receive: 10,
        }
        .data(),
    };

    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new_signed_with_payer(
        &[make_ix],
        Some(&maker.pubkey()),
        &[&maker],
        recent_blockhash,
    );

    // Send the transaction and capture the result
    let tx = svm.send_transaction(transaction).unwrap();

    println!("Tx sign : {:?}", tx);
    // Log transaction details
    msg!("\n\nMake transaction sucessfull");

    // // Check the balance
    // let balance = svm.get_balance(&user.pubkey()).unwrap();
    // assert_eq!(balance, 1_000_000_000);

    // println!("Account funded with {} SOL", balance as f64 / 1e9);
}

/*


 let program_id = address!("Logging111111111111111111111111111111111111");
    let account_meta = AccountMeta {
        pubkey: Address::new_unique(),
        is_signer: false,
        is_writable: true,
    };
    let ix = Instruction {
        program_id,
        accounts: vec![account_meta],
        data: vec![5, 10, 11, 12, 13, 14],
    };
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    let bytes = include_bytes!("../../node-litesvm/program_bytes/spl_example_logging.so");
    svm.add_program(program_id, bytes);
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[payer]).unwrap();
    // let's sim it first
    let sim_res = svm.simulate_transaction(tx.clone()).unwrap();
    let meta = svm.send_transaction(tx).unwrap();
    assert_eq!(sim_res.meta, meta);
    assert_eq!(meta.logs[1], "Program log: static string");
    assert!(meta.compute_units_consumed < 10_000)

*/

fn address_to_pubkey(add: Address) -> Pubkey {
    Pubkey::new_from_array(add.to_bytes())
}

fn pubkey_to_address(pk: Pubkey) -> Address {
    Address::from_str(&pk.to_string()).unwrap()
}
