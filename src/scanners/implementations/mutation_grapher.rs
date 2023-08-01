use syn_solidity::Item;

use crate::scanners::{result::Reporter, Scanner};

#[derive(Default)]
pub struct MutationGrapher {}

impl MutationGrapher {}

impl Scanner for MutationGrapher {
    fn execute(&self, _ast: &[Item], _reporter: &mut Reporter) {
        /*
        println!("todo!")
         */
    }
}
