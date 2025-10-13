use crate::cli::{GenerateCommand, GenerateTarget};
use crate::generator::{
    controller, dto, entity, job, listing, module, processor, service, subscriber, utility::prompt,
};
use anyhow::Result;

pub fn handle(cmd: GenerateCommand) -> Result<()> {
    match cmd.target {
        GenerateTarget::Controller => controller::run(None, None, None)?,
        GenerateTarget::Dto => dto::run(None, None, None)?,
        GenerateTarget::Entity => entity::run(None, None, None)?,
        GenerateTarget::Job => job::run(None, None, None)?,
        GenerateTarget::Listing => listing::run(None, None, None)?,
        GenerateTarget::Module => module::run(None)?,
        GenerateTarget::Processor => processor::run(None, None, None)?,
        GenerateTarget::Service => service::run(None, None, None)?,
        GenerateTarget::Subscriber => subscriber::run(None, None, None)?,
        GenerateTarget::All => {
            let table_name = prompt("Enter table name:")?;
            let entity_name = prompt("Enter entity name:")?;
            let module_name = prompt("Enter module name:")?;

            module::run(Some(&module_name))?;
            controller::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
            dto::run(Some(&entity_name), Some(&module_name), Some("list.filter"))?;
            dto::run(Some(&entity_name), Some(&module_name), Some("find"))?;
            dto::run(Some(&entity_name), Some(&module_name), Some("add"))?;
            entity::run(Some(&table_name), Some(&entity_name), Some(&module_name))?;
            job::run(Some(&entity_name), Some(&module_name), Some(&entity_name))?;
            listing::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
            processor::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
            service::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
            subscriber::run(Some(&entity_name), Some(&entity_name), Some(&module_name))?;
        }
    }
    Ok(())
}
