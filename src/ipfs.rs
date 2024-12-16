use ipfs_api::IpfsClient;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use std::fs::File;
use std::io::Read;
use log::info;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct KnowledgeEntry {
    pub contributor: String,
    pub data: String,
    pub signature: String,
}

#[derive(Error, Debug)]
pub enum IpfsError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("IPFS error: {0}")]
    IpfsApiError(#[from] ipfs_api::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub fn add_knowledge_entry(file_path: &str) -> Result<(), IpfsError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let entry: KnowledgeEntry = serde_json::from_str(&contents)?;

    info!("Adding entry by contributor: {}", entry.contributor);

    let rt = Runtime::new()?;
    rt.block_on(upload_to_ipfs(&entry))
}

async fn upload_to_ipfs(entry: &KnowledgeEntry) -> Result<(), IpfsError> {
    let client = IpfsClient::default();
    let json_data = serde_json::to_string(entry)?;

    let response = client.add(std::io::Cursor::new(json_data)).await?;
    info!("Uploaded to IPFS with hash: {}", response.hash);
    Ok(())
}
