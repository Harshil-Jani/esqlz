use crate::clap_config::{Cli, Commands};
use crate::migrate::{generate_migration, migrate_file, migrate_tui, migration_status, MigrationType};

pub async fn handle_cli(cli: Cli) {
    match &cli.command {
        Commands::Up { file_name } => {
            match file_name {
                Some(file_name) => {
                    migrate_file(file_name, &MigrationType::Up);
                }
                None => {
                    migrate_tui(MigrationType::Up);
                }
            }
        }
        Commands::Down { file_name } => {
            match file_name {
                Some(file_name) => {
                    migrate_file(file_name, &MigrationType::Down);
                }
                None => {
                    migrate_tui(MigrationType::Down);
                }
            }
        },
        Commands::Status => {
            migration_status();
        },
        Commands::Generate { msg } => {
            generate_migration(msg.to_string());
        }
    }
}
