use litesvm::LiteSVM;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;


use crate::counter::account::Count;

anchor_lang::declare_id!(counter)

fn setup () -> (LiteSVM, Keypair) {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    // svm.transfer_lamports(payer.pubkey(), 100_000_000_000).unwrap();
    // (svm, payer)


    let program_bytes: Vec<u8> = std::fs::read("tests/fixtures/counter.so").unwrap();
    svm.add_program(counter, program_bytes);

    svm.airdrop(payer.pubkey(), 100_000_000_000).unwrap();
    (svm, payer)
}

fn count_pda (sender: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"count", sender.as_ref()], &counter)
}


// surfpool run increment --env localnet -u

/*


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountsMap {
    #[serde(flatten)]
    map: HashMap<String, SnapshotAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotAccount {
    pub lamports: u64,
    pub data: String,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}
*/