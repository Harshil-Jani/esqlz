use std::process::Command;

use inquire::MultiSelect;

pub enum MigrationType {
    Up,
    Down,
}

pub fn migrate_tui(migration_type: MigrationType) {
    let migrations = match get_migrations() {
        Ok(migrations) => migrations,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let selected_migrations = MultiSelect::new("Select files or Search files : ", migrations).prompt();
    match selected_migrations {
        Ok(selected) => {
            for migration in selected {
                migrate_file(&migration, &migration_type);
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

pub fn migrate_file(file_name: &String, migration_type: &MigrationType) {
    match migration_type {
        MigrationType::Up => {
            let run = Command::new("npx").arg("sequelize-cli").arg("db:migrate").arg("--name").arg(file_name).output();
            match run {
                Ok(output) => {
                    if output.stderr.is_empty() {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        },
        MigrationType::Down => {
            let run = Command::new("npx").arg("sequelize-cli").arg("db:migrate:undo").arg("--name").arg(file_name).output();
            match run {
                Ok(output) => {
                    if output.stderr.is_empty() {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    } else {
                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        },
    }
}

pub fn migration_status() {
    let run = Command::new("npx").arg("sequelize-cli").arg("db:migrate:status").output();
    match run {
        Ok(output) => {
            if output.stderr.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            } else {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

pub fn generate_migration(msg: String) {
    let run = Command::new("npx").arg("sequelize-cli").arg("migration:generate").arg("--name").arg(msg).output();
    match run {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
fn get_migrations() -> Result<Vec<String>,  String> {
    let run = Command::new("ls").arg("-t").arg("migrations").output();
    match run {
        Ok(output) => {
            if output.stderr.is_empty() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines = stdout.lines();
                let mut migrations = Vec::new();
                for line in lines {
                    migrations.push(line.to_string());
                }
                Ok(migrations)
            } else {
                Err(String::from("Cannot find migrations"))
            }
        }
        Err(e) => {
            Err(format!("Failed to get migrations: {}", e))
        }
    }
}
