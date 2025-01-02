pub mod flavour;
pub mod language;

use crate::config::{AppConfig, FlavourConfig, IterationKind};
use crate::renderer::Renderer;
use crate::schema::OpenAPI;
use anyhow::anyhow;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::Write;

#[derive(Parser)]
#[clap(name = "OpenAPI Manager", version)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Run {
        #[arg(short, long)]
        schema: Option<String>,
        #[arg(short, long)]
        flavour: Option<String>,
        #[arg(short, long = "input", value_names = ["key", "value"])]
        inputs: Vec<String>,
    },
    #[clap(subcommand)]
    Flavour(FlavourCommand),
    #[clap(subcommand)]
    Language(LanguageCommand),
}

#[derive(Subcommand)]
pub enum FlavourCommand {
    Create { name: String },
    Pull { name: String },
}

#[derive(Subcommand)]
pub enum LanguageCommand {
    Create { name: String },
    Pull { name: String },
}

pub fn run(config: AppConfig) -> anyhow::Result<()> {
    // Retrieve schema and flavour.
    let schema = OpenAPI::load(&config.schema)?;
    let flavour = FlavourConfig::load(&config.flavour)?;

    println!();
    println!(
        "Will generate based on OpenAPI schema {} using flavour {}:{}...",
        config.schema.bold(),
        config.flavour.blue(),
        flavour.version.unwrap_or("0.0.0".to_string()).green()
    );
    println!();

    // Store input in a hashmap for easy retrieval.
    let mut data = HashMap::new();

    // Populate data hashmap from CLI and flavour inputs.
    if let Some(inputs) = flavour.inputs {
        for (key, input) in &inputs {
            let input_data = config.inputs.get(key);

            // No value, but input is required.
            if input.required && input_data.is_none() {
                return Err(anyhow!("Missing required input parameter \"{key}\"."));
            }

            let key = key.clone();
            if let Some(value) = input_data {
                data.insert(key, value.clone());
            } else if let Some(value) = &input.default {
                data.insert(key, value.clone());
            }
        }
    }

    // Create renderer instance and provide OpenAPI schema.
    let mut renderer = Renderer::new(schema)?;
    for template in flavour.templates.iter() {
        create_dir_all(&template.output_dir)?;

        let input_path = format!("{}/{}", &config.flavour, template.input);

        if let Some(mode) = &template.iteration {
            match mode {
                // Iterate all OpenAPI schemas.
                IterationKind::Schemas => {
                    todo!("Support generating files per schema.");
                }
                // Iterate all OpenAPI paths.
                IterationKind::Paths => {
                    todo!("Support generating files per path.")
                }
            }
        } else {
            // Render template once.
            let output = renderer.render(&input_path, &data, None, None)?;
            let file_name = renderer.render_str(&template.output, &data)?;
            let output_path = format!("{}/{}", template.output_dir, file_name);

            File::create(output_path)?.write_all(output.as_bytes())?;
            println!("Generated {}", template.name.blue());
        }
    }

    Ok(())
}
