mod metadata;
mod scanner;

use self::{metadata::Metadata, scanner::Registry};
use anyhow::Context;
use clap::Parser;
use std::fs;
use syn_solidity::{File, Item};

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
        let ast_root: File =
            syn::parse_str(&code).with_context(|| "Failed to parse code into AST".to_string())?;
        let ast_items: &[Item] = &ast_root.items;
        let metadata = Metadata::new(file_path);
        let scanners_registry = Registry::default();

        scanners_registry
            .get_scanners()
            .iter()
            .for_each(|s| s.execute(ast_items, &metadata));
        Ok(())
    }
}
