use clap::{Arg, Command};
use crate::ipfs::add_knowledge_entry;
use crate::consensus::validate_entry;
use crate::search::search_entries;
use crate::blockchain::mock_send_to_blockchain;

pub fn run_cli() {
    let matches = Command::new("neuragraph")
        .version("0.2.0")
        .about("Decentralized AI Knowledge Graph")
        .subcommand(
            Command::new("add")
                .about("Add a new knowledge entry")
                .arg(Arg::new("file").required(true).help("Path to the JSON file with knowledge entry")),
        )
        .subcommand(
            Command::new("validate")
                .about("Validate a knowledge entry")
                .arg(Arg::new("file").required(true).help("Path to the JSON file with knowledge entry")),
        )
        .subcommand(
            Command::new("search")
                .about("Search knowledge entries")
                .arg(Arg::new("query").required(true).help("Search query string")),
        )
        .subcommand(
            Command::new("send")
                .about("Send data to blockchain")
                .arg(Arg::new("data").required(true).help("Data to send to blockchain")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        let file_path = matches.value_of("file").unwrap();
        if let Err(e) = add_knowledge_entry(file_path) {
            eprintln!("Error adding knowledge entry: {}", e);
        }
    }

    if let Some(matches) = matches.subcommand_matches("validate") {
        let file_path = matches.value_of("file").unwrap();
        if let Err(e) = validate_entry(file_path) {
            eprintln!("Error validating entry: {}", e);
        }
    }

    if let Some(matches) = matches.subcommand_matches("search") {
        let query = matches.value_of("query").unwrap();
        search_entries(query);
    }

    if let Some(matches) = matches.subcommand_matches("send") {
        let data = matches.value_of("data").unwrap();
        mock_send_to_blockchain(data);
    }
}
