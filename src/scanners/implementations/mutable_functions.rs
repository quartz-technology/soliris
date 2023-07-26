use syn_solidity::Item;

use crate::scanners::{memory::Metadata, Scanner};

#[derive(Default)]
pub struct MutableFunctions {}

impl Scanner for MutableFunctions {
    fn execute(&self, _ast: &[Item], _metadata: &Metadata) {
        println!("todo!")
    }
}
