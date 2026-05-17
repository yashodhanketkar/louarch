//! System shell and external application calls
//!
//! This module contains calls to system shells and external
//! applications such as rofi.
use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::process::{Command, Stdio};

/// Run a os command
///
/// This function will run a command and return the status and output.
///
/// # Arguments
/// * `cmd` - Command to run
/// * `args` - Arguments to pass to the command
pub fn run<I, S>(cmd: &str, args: I) -> anyhow::Result<(bool, String)>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(cmd).args(args).output()?;

    let status = output.status.success();

    let message = String::from_utf8_lossy(if status {
        &output.stdout
    } else {
        &output.stderr
    })
    .trim()
    .to_string();

    Ok((status, message))
}

/// Run a shell commands
///
/// This function will run a shell command.
///
/// # Arguments
/// * `cmd` - Command to run
/// * `args` - Arguments to pass to the command
pub fn cmd_run<I, S>(cmd: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let status = Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("Command `{}` failed with {}", cmd, status);
    }

    Ok(())
}

/// Prompt factory for rofi
///
/// Takes a prompt and a list of options and displays the prompt to the
/// user. Custom options can be disabled by setting `no_custom` to true.
///
/// # Arguments
/// * `prompt` - Prompt to display to the user
/// * `options` - List of options to display to the user
/// * `no_custom` - Disable custom options
///
/// # Requirements
/// * rofi must be installed
///
/// # Errors
/// Returns an error if rofi fails
pub fn rofi_prompt<I, S>(
    prompt: &str,
    options: I,
    no_custom: bool,
) -> anyhow::Result<Option<String>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let config_path = shellexpand::full("~/.config/rofi/prompt.rasi")?.to_string();
    let mut cmd = Command::new("rofi");
    cmd.args(["-dmenu", "-config"]);
    cmd.arg(&config_path);
    cmd.args(["-p", prompt]);

    if no_custom {
        cmd.arg("-no-custom");
    }

    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        for opts in options {
            stdin.write_all(opts.as_ref().as_bytes())?;
            stdin.write_all(b"\n")?;
        }
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let choice = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if choice.is_empty() {
        Ok(None)
    } else {
        Ok(Some(choice))
    }
}

/// Run a tmux command
///
/// This function will run a tmux command with respect to tmux status.
/// If the environment variable TMUX is set, the command will remove it
/// from the running environment to prevent conflicts.
///
/// # Arguments
/// * `args` - Arguments to pass to the command
pub fn tmux_cmd<I, S>(args: I) -> std::process::Command
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = std::process::Command::new("tmux");
    cmd.args(args);
    if env::var("TMUX").is_ok() {
        cmd.env("TMUX", "");
    }
    cmd
}

/// Checks if an application is installed
///
/// # Arguments
/// * `app` - Application to check
pub fn is_installed(app: &str) -> bool {
    env::var_os("PATH")
        .and_then(|paths| {
            env::split_paths(&paths).find(|path| {
                let full_path = path.join(app);
                full_path.exists()
            })
        })
        .is_some()
}
