//! Completions command - Generate shell completion scripts

use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};

use lemma_args::Cli;
use lemma_config::GlobalSettings;
use lemma_output::Printer;

pub fn execute(shell: Shell, _settings: &GlobalSettings, printer: &Printer) -> Result<()> {
    let mut cmd = Cli::command();

    generate(shell, &mut cmd, "lemma", &mut printer.stdout());

    Ok(())
}
