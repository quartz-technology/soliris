use crate::scanners::{memory::Metadata, Scanner};
use syn_solidity::{Item, ItemContract, VariableAttribute, VariableDefinition};

#[derive(Default)]
pub struct MutableVariables {}

impl MutableVariables {
    /// Scans all variables from the contract while discarding the other items.
    fn scan_contract(&self, contract: &ItemContract, metadata: &Metadata) {
        for item in &contract.body {
            if let Item::Variable(variable) = item {
                self.scan_variable(variable, metadata)
            }
        }
    }

    /// Reports if a variable is likely to mutate.
    fn scan_variable(&self, variable: &VariableDefinition, metadata: &Metadata) {
        let immutable_variable_attributes = [
            &VariableAttribute::Constant(Default::default()),
            &VariableAttribute::Immutable(Default::default()),
        ];

        for iva in immutable_variable_attributes {
            if variable.attributes.0.get(iva).is_some() {
                return;
            }
        }

        let line = variable.span().start().line;
        let column = variable.span().start().column;

        println!(
            "{:}:{:}:{:} - Variable state can be changed",
            metadata.file_path, line, column
        )
    }
}

impl Scanner for MutableVariables {
    /// Scans every contract and reports variables able to mutate.
    fn execute(&self, ast: &[Item], metadata: &Metadata) {
        for item in ast {
            if let Item::Contract(contract) = item {
                self.scan_contract(contract, metadata)
            }
        }
    }
}
