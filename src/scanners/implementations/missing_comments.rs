use syn_solidity::Item;

use crate::scanners::{memory::Metadata, Scanner};

#[derive(Default)]
pub struct MissingComments {}

impl Scanner for MissingComments {
    fn execute(&self, _ast: &[Item], _metadata: &Metadata) {
        println!("todo!")
    }
}
