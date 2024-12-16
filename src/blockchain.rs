use log::info;

pub fn mock_send_to_blockchain(data: &str) {
    info!("Sending data to blockchain: {}", data);
    println!("Data successfully sent to blockchain: {}", data);
}
