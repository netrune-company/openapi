mod commands;
mod config;
mod renderer;
mod schema;

use clap::Parser;
use colored::Colorize;
use commands::{Arguments, Command};
use config::AppConfig;

fn main() {
    let arguments = Arguments::parse();

    let result = match arguments.command {
        Command::Init => todo!(),
        Command::Run {
            schema,
            flavour,
            inputs,
        } => commands::run(AppConfig::new(schema, flavour, inputs)),
        Command::Flavour(flavour_command) => match flavour_command {
            commands::FlavourCommand::Create { name } => commands::flavour::create(&name),
            commands::FlavourCommand::Pull { name: _ } => todo!(),
        },
        Command::Language(language_command) => match language_command {
            commands::LanguageCommand::Create { name } => commands::language::create(&name),
            commands::LanguageCommand::Pull { name: _ } => todo!(),
        },
    };

    if let Err(error) = result {
        println!("{}", error.to_string().red());
    }
}
