use std::io;

use clap::CommandFactory;
use clap_complete::{generate, shells::{Bash, Elvish, Fish, PowerShell, Zsh}};

use crate::{Cli, CompletionShell};

pub fn generate_completions(shell: CompletionShell) -> io::Result<()> {
    let mut command = Cli::command();
    let mut stdout = io::stdout();

    match shell {
        CompletionShell::Bash => generate(Bash, &mut command, "img-ascii", &mut stdout),
        CompletionShell::Zsh => generate(Zsh, &mut command, "img-ascii", &mut stdout),
        CompletionShell::Fish => generate(Fish, &mut command, "img-ascii", &mut stdout),
        CompletionShell::PowerShell => generate(PowerShell, &mut command, "img-ascii", &mut stdout),
        CompletionShell::Elvish => generate(Elvish, &mut command, "img-ascii", &mut stdout),
    }

    Ok(())
}
