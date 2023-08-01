use syn_solidity::Item;

use crate::scanners::{result::Reporter, Scanner};

#[derive(Default)]
pub struct StructRepacker {}

impl Scanner for StructRepacker {
    fn execute(&self, _ast: &[Item], _reporter: &mut Reporter) {
        /*
        println!("todo!")
         */
    }
}
