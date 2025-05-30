use anyhow::Result;
use std::fs::File;
use std::io::{Read, Write};
use sui_rpc_api::Client as RpcClient;
use sui_types::full_checkpoint_content::CheckpointData;

pub fn load_checkpoint(file_path: &str) -> CheckpointData {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    bcs::from_bytes(&buffer).unwrap()
}

pub async fn download_and_save_checkpoint(checkpoint_number: u64, file_path: &str) -> Result<()> {
    let sui_client = RpcClient::new("http://localhost:9000").unwrap();
    let summary = sui_client.get_checkpoint_summary(checkpoint_number).await?;
    println!("Summary: {:?}", summary);
    let full_checkpoint = sui_client.get_full_checkpoint(checkpoint_number).await?;
    let mut file = File::create(file_path).unwrap();
    let bytes = bcs::to_bytes(&full_checkpoint).unwrap();
    file.write_all(&bytes).unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let test_file = "checkpoint.chk";

    if !std::path::Path::new(test_file).exists() {
        println!("Checkpoint not found, fetching from local network");
        download_and_save_checkpoint(2, test_file).await?;
    }

    println!("Loading checkpoint from file");
    let full_checkpoint = load_checkpoint(test_file);

    println!("Full checkpoint: {:?}", full_checkpoint);
    let commitments = &full_checkpoint
        .checkpoint_summary
        .data()
        .checkpoint_commitments;
    println!("Commitments: {:?}", commitments);

    Ok(())
}
