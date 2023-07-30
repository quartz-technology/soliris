pub mod implementations;
pub mod memory;

use self::implementations::{
    missing_comments::MissingComments, mutable_functions::MutableFunctions,
    mutable_variables::MutableVariables, mutation_grapher::MutationGrapher,
    struct_repacker::StructRepacker, unused_imports::UnusedImports,
};
use crate::scanners::memory::Metadata;
use syn_solidity::Item;

pub trait Scanner {
    fn execute(&self, ast: &[Item], metadata: &Metadata);
}

pub struct Registry {
    scanners: Vec<Box<dyn Scanner>>,
}

impl Registry {
    #[allow(dead_code)]
    pub fn register_scanner(&mut self, scanner: Box<dyn Scanner>) {
        self.scanners.push(scanner)
    }

    pub fn get_scanners(&self) -> &[Box<dyn Scanner>] {
        &self.scanners
    }
}

impl Default for Registry {
    fn default() -> Self {
        Registry {
            scanners: vec![
                Box::<MissingComments>::default(),
                Box::<UnusedImports>::default(),
                Box::<MutableVariables>::default(),
                Box::<MutableFunctions>::default(),
                Box::<StructRepacker>::default(),
                Box::<MutationGrapher>::default(),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Metadata, Registry, Scanner};
    use syn_solidity::Item;

    #[derive(Default)]
    struct MockScanner {}

    impl Scanner for MockScanner {
        fn execute(&self, _ast: &[Item], _metadata: &Metadata) {
            todo!()
        }
    }

    #[test]
    fn it_creates_default_scanners_registry() {
        let scanners_registry = Registry::default();

        assert_eq!(scanners_registry.get_scanners().len(), 6)
    }

    #[test]
    fn it_registers_a_new_scanner() {
        let mut scanners_registry = Registry::default();

        scanners_registry.register_scanner(Box::new(MockScanner::default()));

        assert_eq!(scanners_registry.get_scanners().len(), 7)
    }
}
