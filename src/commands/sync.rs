use crate::cli::{SyncCommand, SyncTarget};
use crate::sync::{column_mappings, entity_hash, job_hash, module_components};
use anyhow::Result;

pub fn handle(cmd: SyncCommand) -> Result<()> {
    match cmd.target {
        SyncTarget::Entities => entity_hash::run()?,
        SyncTarget::Jobs => job_hash::run()?,
        SyncTarget::Components => module_components::run()?,
        SyncTarget::ColumnMappings => column_mappings::run()?,
        SyncTarget::All => {
            entity_hash::run()?;
            job_hash::run()?;
            module_components::run()?;
            column_mappings::run()?;
        }
    }
    Ok(())
}
