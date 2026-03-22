use core::option::Option::Some;

use crate::cli::{GenerateCommand, GenerateTarget};
use crate::generator::{
    controller, dto, entity, job, listing, module, processor, service, subscriber, utility::prompt,
};
use anyhow::Result;
use colored::*;

pub fn handle(cmd: GenerateCommand) -> Result<()> {
    match cmd.target {
        GenerateTarget::Controller => controller::run(None, None, None, None)?,
        GenerateTarget::Dto => dto::run(None, None, None, None)?,
        GenerateTarget::Entity => entity::run(None, None, None)?,
        GenerateTarget::Job => job::run(None, None, None)?,
        GenerateTarget::Listing => listing::run(None, None, None, None)?,
        GenerateTarget::Module => module::run(None)?,
        GenerateTarget::Processor => processor::run(None, None, None, None)?,
        GenerateTarget::Service => service::run(None, None, None, None)?,
        GenerateTarget::Subscriber => subscriber::run(None, None, None)?,
        GenerateTarget::All => {
            let table_name = prompt("Enter table name:")?;
            if table_name.trim().is_empty() {
                eprintln!(
                    "\n{}\n",
                    "✖ Error: Table name is a mandatory field.".red().bold()
                );
                std::process::exit(1);
            }
            let entity_name = prompt("Enter entity name:")?;
            if entity_name.trim().is_empty() {
                eprintln!(
                    "\n{}\n",
                    "✖ Error: Entity name is a mandatory field.".red().bold()
                );
                std::process::exit(1);
            }
            let module_name = prompt("Enter module name:")?;
            if module_name.trim().is_empty() {
                eprintln!(
                    "\n{}\n",
                    "✖ Error: Module name is a mandatory field.".red().bold()
                );
                std::process::exit(1);
            }

            let generate_controller = {
                let input = prompt("Generate controller? (Y/n):")?;
                input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y")
            };

            let mut generate_listing = true;
            let mut generate_processor = true;
            let mut entity_type = "both";
            if generate_controller {
                let input = prompt("Generate listing? (Y/n):")?;
                generate_listing =
                    input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y");

                let input = prompt("Generate processor? (Y/n):")?;
                generate_processor =
                    input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y");

                let input = prompt(
                    "Select entity type to generate:\n1) Business specific\n2) User specific\n3) Both (default)\nEnter choice (1/2/3):",
                )?;
                entity_type = match input.trim() {
                    "1" => "business",
                    "2" => "user",
                    _ => "both",
                };
            }

            module::run(Some(&module_name))?;
            entity::run(Some(&table_name), Some(&entity_name), Some(&module_name))?;
            job::run(Some(&entity_name), Some(&module_name), Some(&entity_name))?;
            subscriber::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;

            if generate_controller {
                controller::run(
                    Some(&entity_name),
                    Some(&entity_name),
                    Some(&module_name),
                    Some(entity_type),
                )?;
                dto::run(
                    Some(&entity_name),
                    Some(&module_name),
                    Some("find"),
                    Some(entity_type),
                )?;
                service::run(
                    Some(&entity_name),
                    Some(&entity_name),
                    Some(&module_name),
                    Some(entity_type),
                )?;

                if generate_listing {
                    dto::run(
                        Some(&entity_name),
                        Some(&module_name),
                        Some("list.filter"),
                        Some(entity_type),
                    )?;
                    listing::run(
                        Some(&entity_name),
                        Some(&entity_name),
                        Some(&module_name),
                        Some(entity_type),
                    )?;
                }

                if generate_processor {
                    dto::run(
                        Some(&entity_name),
                        Some(&module_name),
                        Some("add"),
                        Some(entity_type),
                    )?;
                    processor::run(
                        Some(&entity_name),
                        Some(&entity_name),
                        Some(&module_name),
                        Some(entity_type),
                    )?;
                }
            }
        }
    }
    Ok(())
}
