use color_eyre::eyre::Result;
mod gp_tasks;
mod init;

fn main() -> Result<()> {
    init::tracing()?;
    color_eyre::install()?;
    gp_tasks::attach()?;

    Ok(())
}
