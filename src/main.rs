mod cli;
mod commands;
mod scanners;

fn main() -> Result<(), anyhow::Error> {
    cli::run()
}
