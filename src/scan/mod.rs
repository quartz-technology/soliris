use anyhow::Context;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
pub struct Command {
    #[arg(short = 'f', long = "file")]
    file_path: String,
}

impl Command {
    pub fn execute(self) -> Result<(), anyhow::Error> {
        let file_path = &self.file_path;
        let code = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read content from {}", file_path))?;

        println!("{:?}", code);
        Ok(())
    }
}
