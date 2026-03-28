mod cli;
mod commands;
mod data;

fn main() -> anyhow::Result<()> {
    cli::run()
}
