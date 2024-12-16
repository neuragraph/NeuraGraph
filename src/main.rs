mod cli;
mod ipfs;
mod consensus;
mod crypto_utils;
mod utils;
mod blockchain;
mod search;

use cli::run_cli;

fn main() {
    env_logger::init();
    run_cli();
}
