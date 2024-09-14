pub mod clap_config;
pub mod cli;
pub mod setup;
pub mod migrate;

use clap::Parser;
use clap_config::Cli;
use cli::handle_cli;

#[tokio::main]
async fn main() {
    match setup::check_setup() {
        Ok(_) => {
            let cli = Cli::parse();
            handle_cli(cli).await;
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
