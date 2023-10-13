use crate::scanners::{
    result::{Reporter, Severity},
    Scanner,
};
use syn_solidity::{Item, ItemContract, Spanned, VariableAttribute, VariableDefinition};

#[derive(Default)]
pub struct MutableVariables {}

impl MutableVariables {
    /// Scans all variables from the contract while discarding the other items.
    fn scan_contract(&self, contract: &ItemContract, reporter: &mut Reporter) {
        for item in &contract.body {
            if let Item::Variable(variable) = item {
                self.scan_variable(variable, reporter)
            }
        }
    }

    /// Reports if a variable is likely to mutate.
    fn scan_variable(&self, variable: &VariableDefinition, reporter: &mut Reporter) {
        let immutable_variable_attributes = [
            &VariableAttribute::Constant(Default::default()),
            &VariableAttribute::Immutable(Default::default()),
        ];

        for iva in immutable_variable_attributes {
            if variable.attributes.0.contains(iva) {
                return;
            }
        }

        let line = variable.span().start().line;
        let column = variable.span().start().column;

        reporter.report(
            line,
            column,
            Severity::Info,
            "Variable state can be changed",
        )
    }
}

impl Scanner for MutableVariables {
    /// Scans every contract and reports variables able to mutate.
    fn execute(&self, ast: &[Item], reporter: &mut Reporter) {
        for item in ast {
            if let Item::Contract(contract) = item {
                self.scan_contract(contract, reporter)
            }
        }
    }
}
