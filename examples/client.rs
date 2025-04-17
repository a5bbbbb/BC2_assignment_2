use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::fs::File;
use std::str::FromStr;
use std::io::Read;
use std::env;
use std::path;
use std::fs;
use serde_json;
fn load_default_keypair() -> Keypair {
    let home_path = env::var_os("HOME").unwrap();
    let default_keypair_path = ".config/solana/id.json"; // ! update if you want to use a different path
    let default_keypair_path = path::PathBuf::from(home_path).join(default_keypair_path);

    let keypair_file = fs::read_to_string(default_keypair_path).unwrap();
    let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_file).unwrap();
    let default_keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
    println!("loaded keypair address -> {:?}", default_keypair.pubkey()); // ! debug

    default_keypair
}
#[tokio::main]
async fn main() {
    // Program ID (replace with your actual program ID)
    let program_id = Pubkey::from_str("8LVMqMuEpNq4Sng8SirxdYvDgTyxd5CkgmqsmEth2trY").unwrap();

    // Connect to the Solana devnet
    let rpc_url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate a new keypair for the payer
    let payer = load_default_keypair();
    // Create the instruction
    let instruction = Instruction::new_with_borsh(
        program_id,
        &(),    // Empty instruction data
        vec![], // No accounts needed
    );

    // Add the instruction to new transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], client.get_latest_blockhash().unwrap());

    // Send and confirm the transaction
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}