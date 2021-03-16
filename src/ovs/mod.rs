use std::process::Command;

fn run_command(cmd: Vec<&str>) -> anyhow::Result<()> {

    let program = cmd.first().expect("Requires cmd");
    let args = &cmd[1..];

    Command::new(program)
        .args(args)
        .output()?;
    Ok(())
}
