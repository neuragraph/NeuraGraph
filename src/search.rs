use log::info;

pub fn search_entries(query: &str) {
    info!("Searching for entries matching: {}", query);
    println!("No entries found for query: '{}'", query);
}
