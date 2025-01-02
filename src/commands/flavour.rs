use colored::Colorize;
use std::fs::{create_dir_all, File};

pub fn create(name: &str) -> anyhow::Result<()> {
    create_dir_all(format!(".openapi/flavours/{name}"))?;
    File::create_new(format!(".openapi/flavours/{name}/config.yaml"))?;

    println!();
    println!(
        "Created new flavour {} under {} 🎉",
        name.blue(),
        format!(".openapi/flavours/{}", name).bold()
    );
    println!("You can now start implementing your flavour.");
    println!();
    println!(" 1. Begin by adding templates under the generated directory.",);
    println!(
        " 2. Modify the {} file and map a template to an output file.",
        "config.yaml".bold()
    );
    println!(" 3. Optionally, you can provide a processor in the form of a WASM file.");
    println!();
    println!(
        "Read more at {}",
        "http://localhost:8080/how_to_wasm".bold().underline()
    );
    println!();

    Ok(())
}
