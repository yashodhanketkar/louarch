use clap_complete::Shell;

use crate::tools::completions::_internal::{atomic_writer, generator};

/// Create and store bash completions
///
/// # Arguments
/// * `path` - Path to completions directory
///
/// # Errors
/// Returns an error if the completions cannot be generated.
pub(crate) fn handle(path: String) -> anyhow::Result<()> {
    let file_path = format!("{}/completion.bash", path);
    let generated = generator(Shell::Bash)?;
    atomic_writer(&file_path, &generated)?;

    println!(
        "Completions installed to {}.
Please add following code to your .bashrc file:

`source ~/.cache/louarch/completion.bash`",
        file_path
    );

    Ok(())
}
