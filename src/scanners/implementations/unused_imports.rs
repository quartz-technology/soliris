use syn_solidity::Item;

use crate::scanners::{memory::Metadata, Scanner};

#[derive(Default)]
pub struct UnusedImports {}

impl Scanner for UnusedImports {
    fn execute(&self, _ast: &[Item], _metadata: &Metadata) {
        println!("todo!")
    }
}
