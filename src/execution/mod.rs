mod local;
mod slurm;

use crate::setup::instance::Instance;
use anyhow::Result;
use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Default, ValueEnum)]
pub enum ExecutionKind {
    #[default]
    Local,
    Slurm,
}

pub fn execute(instance: Instance, kind: ExecutionKind, threads: usize) -> Result<()> {
    match kind {
        ExecutionKind::Local => local::execute(instance, threads),
        ExecutionKind::Slurm => slurm::execute(instance),
    }
}
