use anyhow::Result;
use sui_sdk::rpc_types::Checkpoint;
// use std::fs::File;
// use std::io::{Read, Write};
use sui_rpc_api::Client as RpcClient;
use sui_types::full_checkpoint_content::CheckpointData;
use sui_sdk::SuiClientBuilder;
use sui_json_rpc_types::CheckpointId;

// pub fn load_checkpoint(file_path: &str) -> CheckpointData {
//     let mut file = File::open(file_path).unwrap();
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).unwrap();
//     bcs::from_bytes(&buffer).unwrap()
// }

// pub async fn download_and_save_checkpoint(checkpoint_number: u64, file_path: &str) -> Result<()> {
//     let sui_client = RpcClient::new("http://localhost:9000").unwrap();
//     let full_checkpoint = sui_client.get_full_checkpoint(checkpoint_number).await?;
//     let mut file = File::create(file_path).unwrap();
//     let bytes = bcs::to_bytes(&full_checkpoint).unwrap();
//     file.write_all(&bytes).unwrap();
//     Ok(())
// }

pub async fn get_checkpoint_via_grpc(checkpoint_number: u64) -> Result<CheckpointData> {
    let sui_client = RpcClient::new("http://localhost:9000").unwrap();
    let checkpoint = sui_client.get_full_checkpoint(checkpoint_number).await?;
    Ok(checkpoint)
}

pub async fn get_checkpoint_via_sdk(checkpoint_number: u64) -> Result<Checkpoint> {
    let sui_localnet = SuiClientBuilder::default().build_localnet().await?;
    let checkpoint = sui_localnet
        .read_api()
        .get_checkpoint(CheckpointId::SequenceNumber(checkpoint_number))
        .await?;
    Ok(checkpoint)
}

#[tokio::main]
async fn main() -> Result<()> {
    let use_sdk = false;
    let checkpoint_number = 2;
    println!("Reading checkpoint {}", checkpoint_number);
    if use_sdk {
        println!("Using SDK");
        let checkpoint = get_checkpoint_via_sdk(checkpoint_number).await?;
        println!("Checkpoint: {:?}", checkpoint);
        let commitments = &checkpoint.checkpoint_commitments;
        println!("Commitments: {:?}", commitments);
    } else {
        println!("Using GRPC");
        let checkpoint = get_checkpoint_via_grpc(checkpoint_number).await?;
        println!("Checkpoint: {:?}", checkpoint);
        let commitments = &checkpoint.checkpoint_summary.data().checkpoint_commitments;
        println!("Commitments: {:?}", commitments);
    }

    // let test_file = format!("test_files/checkpoint-{}.chk", checkpoint_number);

    // if !std::path::Path::new(&test_file).exists() {
    //     println!(
    //         "Checkpoint {} not found, fetching from local network",
    //         checkpoint_number
    //     );
    //     download_and_save_checkpoint(checkpoint_number, &test_file).await?;
    // }

    // println!("Loading checkpoint from file {}", test_file);
    // let full_checkpoint = load_checkpoint(&test_file);
    // let summary = full_checkpoint.checkpoint_summary.data();
    // println!("Summary: {:?}", summary);

    // let commitments = &full_checkpoint
    //     .checkpoint_summary
    //     .data()
    //     .checkpoint_commitments;
    // println!("Commitments: {:?}", commitments);

    Ok(())
}
