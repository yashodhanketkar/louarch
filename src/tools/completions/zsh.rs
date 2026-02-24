use clap_complete::Shell;

use crate::tools::completions::_internal::{atomic_writer, generator};

/// Create and store zsh completions
///
/// # Arguments
/// * `path` - Path to completions directory
/// * `silent` - Whether to print the completions to stdout
///
/// # Errors
/// Returns an error if the completions cannot be generated.
pub(crate) fn handle(path: String, silent: bool) -> anyhow::Result<()> {
    let file_path = format!("{}/completion.zsh", path);
    let generated = generator(Shell::Zsh)?;
    atomic_writer(&file_path, &generated)?;

    if !silent {
        println!(
            "Completions installed to {}.
Please add following code to your .zshrc file:

`source ~/.cache/louarch/completion.zsh`",
            file_path
        );
    }

    Ok(())
}
