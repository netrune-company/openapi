use colored::Colorize;
use std::fs::{create_dir_all, File};

pub fn create(name: &str) -> anyhow::Result<()> {
    create_dir_all(format!(".openapi/languages/{name}"))?;
    File::create_new(format!(".openapi/languages/{name}/config.yaml"))?;

    println!();
    println!(
        "Created new language {} under {} 🎉",
        name.blue(),
        format!(".openapi/language/{}", name).bold()
    );
    println!("You can now start implementing your language.");
    println!();

    Ok(())
}
