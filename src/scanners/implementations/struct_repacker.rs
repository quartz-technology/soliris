use syn_solidity::Item;

use crate::scanners::{memory::Metadata, Scanner};

#[derive(Default)]
pub struct StructRepacker {}

impl Scanner for StructRepacker {
    fn execute(&self, _ast: &[Item], _metadata: &Metadata) {
        println!("todo!")
    }
}
