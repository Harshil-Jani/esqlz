use std::{error::Error, process::Command};

pub fn check_setup() -> Result<(), Box<dyn Error>> {
    // Check if sequelize is installed already
    let output = Command::new("npx").arg("sequelize-cli").arg("--version").output();
    if output.is_err() {
        return Err("Sequelize is not installed. Please install sequelize-cli first".into());
    }
    Ok(())
}
