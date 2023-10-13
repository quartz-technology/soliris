use crate::scanners::{
    result::{Reporter, Severity},
    Scanner,
};
use syn_solidity::{FunctionAttribute, Item, ItemContract, ItemFunction, Mutability, Spanned};

#[derive(Default)]
pub struct MutableFunctions {}

impl MutableFunctions {
    /// Scans all functions from the contract while discarding the other items.
    fn scan_contract(&self, contract: &ItemContract, reporter: &mut Reporter) {
        for item in &contract.body {
            if let Item::Function(function) = item {
                self.scan_function(function, reporter)
            }
        }
    }

    /// Reports if a function is able to mutate the contract state.
    fn scan_function(&self, function: &ItemFunction, reporter: &mut Reporter) {
        if function.kind.is_modifier() {
            return;
        }

        let immutable_function_attributes = [
            &FunctionAttribute::Mutability(Mutability::Pure(Default::default())),
            &FunctionAttribute::Mutability(Mutability::View(Default::default())),
            &FunctionAttribute::Mutability(Mutability::Constant(Default::default())),
        ];

        for ifa in immutable_function_attributes {
            if function.attributes.contains(ifa) {
                return;
            }
        }

        let line = function.span().start().line;
        let column = function.span().start().column;

        reporter.report(
            line,
            column,
            Severity::Info,
            "Function can mutate contract state",
        )
    }
}

impl Scanner for MutableFunctions {
    /// Scans every contract and reports functions able to mutate the storage state.
    fn execute(&self, ast: &[Item], reporter: &mut Reporter) {
        for item in ast {
            // Mutable functions are only located inside contracts.
            if let Item::Contract(contract) = item {
                self.scan_contract(contract, reporter)
            }
        }
    }
}
