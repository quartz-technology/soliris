use syn_solidity::Item;

use crate::scanners::{result::Reporter, Scanner};

#[derive(Default)]
pub struct UnusedImports {}

impl Scanner for UnusedImports {
    fn execute(&self, _ast: &[Item], _reporter: &mut Reporter) {
        /*
        println!("todo!")
         */
    }
}
