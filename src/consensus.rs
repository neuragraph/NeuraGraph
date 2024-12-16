use crate::ipfs::KnowledgeEntry;
use std::fs::File;
use std::io::Read;
use serde_json;
use log::info;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("File error: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] serde_json::Error),
}

pub fn validate_entry(file_path: &str) -> Result<(), ValidationError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let entry: KnowledgeEntry = serde_json::from_str(&contents)?;

    info!("Validating entry by contributor: {}", entry.contributor);

    if entry.data.is_empty() {
        eprintln!("Validation failed: data is empty.");
        return Err(ValidationError::DeserializationError(serde_json::Error::custom("Data is empty")));
    }

    info!("Entry validated successfully.");
    Ok(())
}
