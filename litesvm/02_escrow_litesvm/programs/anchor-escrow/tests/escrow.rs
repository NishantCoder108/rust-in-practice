use {
    anchor_escrow::{
        accounts::{Make as MakeAccounts, Refund, Take},
        instruction::{Make as MakeIx, Refund as RefundIx, Take as TakeIx},
        program::AnchorEscrow,
        state::Escrow,
    },
    anchor_lang::{
        prelude::msg, AccountDeserialize, AnchorDeserialize, InstructionData, ToAccountMetas,
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
    // solana_message::{Message, VersionedMessage},
    // solana_native_token::LAMPORTS_PER_SOL,
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
        .authority(&taker.pubkey())
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

    let escrow_account = svm.get_account(&pubkey_to_address(escrow)).unwrap();
    println!("Escrow Account : {:?}", escrow_account);

    // let escrow_data = MakeIx::deserialize(&mut escrow_account.data.as_ref()).unwrap();
    let escrow_state = Escrow::try_deserialize(&mut escrow_account.data.as_slice()).unwrap();

    // // Check the balance
    // let balance = svm.get_balance(&user.pubkey()).unwrap();
    // assert_eq!(balance, 1_000_000_000);

    println!("{:?}", escrow_state);

    println!("seed: {}", escrow_state.seed);
    // println!("deposit: {}", escrow_data.deposit);
    println!("receive: {}", escrow_state.receive);
    // assert_eq!(10u64, escrow_data.deposit, "Escrow data deposit value");
    assert_eq!(123, escrow_state.seed, "Escrow data seed");
    assert_eq!(
        10u64, escrow_state.receive,
        "Escrow expected received value"
    );

    /*Tests for taker */
    // Mint 1,000 tokens (with 6 decimal places) of Mint A to the maker's associated token account
    MintTo::new(&mut svm, &taker, &mint_b, &taker_ata_b, 1000000000)
        .send()
        .unwrap();

    let take_ix = Instruction {
        // `Instruction::program_id` is an `Address` in this SDK
        program_id,
        accounts: Take {
            taker: address_to_pubkey(taker.pubkey()),
            maker: address_to_pubkey(maker.pubkey()),
            mint_a: address_to_pubkey(mint_a),
            mint_b: address_to_pubkey(mint_b),
            taker_ata_a: address_to_pubkey(taker_ata_a),
            taker_ata_b: address_to_pubkey(taker_ata_b),
            maker_ata_b: address_to_pubkey(maker_ata_b),
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
        data: TakeIx {}.data(),
    };

    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new_signed_with_payer(
        &[take_ix],
        Some(&taker.pubkey()),
        &[&taker],
        recent_blockhash,
    );

    // Send the transaction and capture the result
    let tx = svm.send_transaction(transaction).unwrap();

    println!("Tx sign : {:?}", tx);
    // Log transaction details
    msg!("\n\nTake transaction sucessfull");

    // After take, escrow account should not exist anymore (should be None)
    let escrow_addr = pubkey_to_address(escrow);
    let escrow_account = svm.get_account(&escrow_addr);

    if escrow_account.is_none() {
        println!("Escrow account does not exist anymore after take. (as expected)");
    } else {
        println!("Escrow account still exists: {:?}", escrow_account);
    }

    assert!(escrow_account.is_none(), "Vault is not exist anymore");

    /* Refund test */

    //Again maker for refund
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
            deposit: 100,
            seed: 123u64,
            receive: 100,
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
    msg!("\n\nAgain Maker transaction sucessfull");
    let refund_ix = Instruction {
        // `Instruction::program_id` is an `Address` in this SDK
        program_id,
        accounts: Refund {
            // taker: address_to_pubkey(taker.pubkey()),
            maker: address_to_pubkey(maker.pubkey()),
            mint_a: address_to_pubkey(mint_a),
            maker_ata_a: address_to_pubkey(maker_ata_a),
            escrow: escrow,
            vault: vault,
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
        data: RefundIx {}.data(),
    };

    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new_signed_with_payer(
        &[refund_ix],
        Some(&maker.pubkey()),
        &[&maker],
        recent_blockhash,
    );

    // Send the transaction and capture the result
    let tx = svm.send_transaction(transaction).unwrap();

    println!("Tx sign : {:?}", tx);
    // Log transaction details
    msg!("\n\nRefund transaction sucessfull");

    // Fetch the updated escrow account to verify it was closed/refunded
    let escrow_account_post_refund = svm.get_account(&pubkey_to_address(escrow));
    assert!(
        escrow_account_post_refund.is_none()
            || escrow_account_post_refund.as_ref().unwrap().lamports == 0,
        "Escrow account should be closed or have zero lamports after refund"
    );

    // Check that the maker's associated token account received the refund (balance increased)
    let ata_account = svm.get_account(&(maker_ata_a)).unwrap();
    let ata_balance =
        anchor_spl::token::TokenAccount::try_deserialize(&mut ata_account.data.as_slice())
            .unwrap()
            .amount;
    assert_eq!(
        ata_balance,
        1000000000 - 10,
        "Maker's ATA should have 1,000 tokens after refund"
    );
}

fn address_to_pubkey(add: Address) -> Pubkey {
    Pubkey::new_from_array(add.to_bytes())
}

fn pubkey_to_address(pk: Pubkey) -> Address {
    Address::from_str(&pk.to_string()).unwrap()
}
