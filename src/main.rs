mod cli;
mod scan;

fn main() -> Result<(), anyhow::Error> {
    cli::run()
}
