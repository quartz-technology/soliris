use crate::scanners::{memory::Metadata, Scanner};
use syn_solidity::{FunctionAttribute, Item, ItemContract, ItemFunction, Mutability};

#[derive(Default)]
pub struct MutableFunctions {}

impl MutableFunctions {
    /// Scans all functions from the contract while discarding the other items.
    fn scan_contract(&self, contract: &ItemContract, metadata: &Metadata) {
        for item in &contract.body {
            if let Item::Function(function) = item {
                self.scan_function(function, metadata)
            }
        }
    }

    /// Reports if a function is able to mutate the contract state.
    fn scan_function(&self, function: &ItemFunction, metadata: &Metadata) {
        // TODO: There is probably a cleaner way to check this.
        if function.kind.as_str() == "modifier" {
            return;
        }

        let immutable_function_attributes = [
            &FunctionAttribute::Mutability(Mutability::Pure(Default::default())),
            &FunctionAttribute::Mutability(Mutability::View(Default::default())),
            &FunctionAttribute::Mutability(Mutability::Constant(Default::default())),
        ];

        for ifa in immutable_function_attributes {
            if function.attributes.get(ifa).is_some() {
                return;
            }
        }

        let line = function.span().start().line;
        let column = function.span().start().column;

        println!(
            "{:}:{:}:{:} - Function can mutate contract state",
            metadata.file_path, line, column
        )
    }
}

impl Scanner for MutableFunctions {
    /// Scans every contract and reports functions able to mutate the storage state.
    fn execute(&self, ast: &[Item], metadata: &Metadata) {
        for item in ast {
            // Mutable functions are only located inside contracts.
            if let Item::Contract(contract) = item {
                self.scan_contract(contract, metadata)
            }
        }
    }
}
