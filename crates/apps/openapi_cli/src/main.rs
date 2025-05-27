use std::process::exit;

use clap::Parser;
use openapi_kit_renderer::Renderer;
use openapi_kit_workspace::Workspace;

#[derive(Parser)]
struct Config {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Generate { reference: String },
}

fn main() {
    let config = Config::parse();

    let workspace = match Workspace::load() {
        Ok(workspace) => workspace,
        Err(e) => {
            eprintln!("Failed to load workspace: {e}");
            exit(1)
        }
    };

    match config.command {
        Command::Generate { reference } => {
            let mut segments = reference.split(':');

            let Some(project_name) = segments.next() else {
                return;
            };

            let Some(project) = workspace.config.projects.get(project_name) else {
                eprintln!("Project '{project_name}' not found in workspace.");
                exit(1);
            };

            // Set fallback for schema path, and load the schema
            let schema_path = workspace.path.join(&project.schema_path);
            let schema = match openapi_kit_schema::load(&schema_path) {
                Ok(schema) => schema,
                Err(error) => panic!(
                    "Failed to load schema at {}: {:?}",
                    schema_path.display(),
                    error
                ),
            };
            let Ok(renderer) = Renderer::new(&workspace, project_name) else {
                eprintln!("Failed to create renderer for project '{project_name}'.");
                exit(1);
            };

            if let Some(template_name) = segments.next() {
                let Some(template) = project.templates.get(template_name) else {
                    eprintln!("Template '{template_name}' not found in project '{project_name}'.");
                    exit(1);
                };

                let Ok(output) = renderer.render(&template.path, &schema.into()) else {
                    eprintln!(
                        "Failed to render template '{template_name}' in project '{project_name}'."
                    );
                    exit(1);
                };

                if std::fs::write(workspace.path.join(&template.output), &output).is_err() {
                    eprintln!(
                        "Failed to write output for template '{template_name}' in project '{project_name}'."
                    );
                    exit(1);
                }
            } else {
                for (template_name, template) in &project.templates {
                    let Ok(output) = renderer.render(&template.path, &schema.clone().into()) else {
                        eprintln!(
                            "Failed to render template '{template_name}' in project '{project_name}'."
                        );
                        exit(1);
                    };

                    if std::fs::write(workspace.path.join(&template.output), &output).is_err() {
                        eprintln!(
                            "Failed to write output for template '{template_name}' in project '{project_name}'."
                        );
                        exit(1);
                    }
                }
            }
        }
    }
}
