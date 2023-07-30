use syn_solidity::{Item, ItemContract, ItemFunction};

use crate::scanners::{memory::Metadata, Scanner};

#[derive(Default)]
pub struct MutationGrapher {}

impl MutationGrapher {}

impl Scanner for MutationGrapher {
    fn execute(&self, _ast: &[Item], _metadata: &Metadata) {
        println!("todo!")
    }
}
