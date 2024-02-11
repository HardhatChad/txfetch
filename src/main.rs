use std::env;
use std::str::FromStr;

use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;

const RPC_URL: &str = "TODO";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: program <transaction_signature>");
        return;
    }
    let client = RpcClient::new_with_commitment(RPC_URL.into(), CommitmentConfig::processed());
    let sig = Signature::from_str(&args[1]).unwrap();
    match client
        .get_transaction(&sig, UiTransactionEncoding::JsonParsed)
        .await
    {
        Ok(tx) => println!("{:#?}", tx),
        Err(e) => eprintln!("Error fetching transaction: {:?}", e),
    }
}
