use syn_solidity::{
    Item, ItemContract, ItemEnum, ItemError, ItemEvent, ItemFunction, ItemStruct, ItemUdt,
};

use crate::scanners::{memory::Metadata, Scanner};

#[derive(Default)]
pub struct MissingComments {}

impl MissingComments {
    fn scan_in_contract(&self, contract: &ItemContract, metadata: &Metadata) {
        for item in &contract.body {
            match item {
                Item::Enum(enumeration) => {
                    self.check_missing_comments_for_enum(enumeration, metadata)
                }
                Item::Error(error) => self.check_missing_comments_for_error(error, metadata),
                Item::Event(event) => self.check_missing_comments_for_event(event, metadata),
                Item::Function(function) => {
                    self.check_missing_comments_for_function(function, metadata)
                }
                Item::Struct(structure) => {
                    self.check_missing_comments_for_structure(structure, metadata)
                }
                Item::Udt(udt) => self.check_missing_comments_for_udt(udt, metadata),
                /* Contracts can not be declared inside other contracts */
                /* Import directives don't have attributes. */
                /* Pragma directives don't have attributes. */
                /* Variable attributes don't include comments. */
                /* Using declarations don't need to be commented. */
                _ => {}
            }
        }
    }

    fn check_missing_comments_for_contract(&self, contract: &ItemContract, metadata: &Metadata) {
        if contract.attrs.is_empty() {
            let line = contract.span().start().line;
            let column = contract.span().start().column;

            println!(
                "{:}:{:}:{:} - Missing comment on contract definition",
                metadata.file_path, line, column
            )
        }
    }

    fn check_missing_comments_for_enum(&self, enumeration: &ItemEnum, metadata: &Metadata) {
        if enumeration.attrs.is_empty() {
            let line = enumeration.span().start().line;
            let column = enumeration.span().start().column;

            println!(
                "{:}:{:}:{:} - Missing comment on enum definition",
                metadata.file_path, line, column
            )
        }
    }

    fn check_missing_comments_for_event(&self, event: &ItemEvent, metadata: &Metadata) {
        if event.attrs.is_empty() {
            let line = event.span().start().line;
            let column = event.span().start().column;

            println!(
                "{:}:{:}:{:} - Missing comment on event definition",
                metadata.file_path, line, column
            )
        }
    }

    fn check_missing_comments_for_error(&self, error: &ItemError, metadata: &Metadata) {
        if error.attrs.is_empty() {
            let line = error.span().start().line;
            let column = error.span().start().column;

            println!(
                "{:}:{:}:{:} - Missing comment on error definition",
                metadata.file_path, line, column
            )
        }
    }

    fn check_missing_comments_for_function(&self, function: &ItemFunction, metadata: &Metadata) {
        if function.attrs.is_empty() {
            let line = function.span().start().line;
            let column = function.span().start().column;

            println!(
                "{:}:{:}:{:} - Missing comment for function definition",
                metadata.file_path, line, column
            )
        }
    }

    fn check_missing_comments_for_structure(&self, structure: &ItemStruct, metadata: &Metadata) {
        if structure.attrs.is_empty() {
            let line = structure.span().start().line;
            let column = structure.span().start().column;

            println!(
                "{:}:{:}:{:} - Missing comment for structure definition",
                metadata.file_path, line, column
            )
        }
    }

    fn check_missing_comments_for_udt(&self, udt: &ItemUdt, metadata: &Metadata) {
        if udt.attrs.is_empty() {
            let line = udt.span().start().line;
            let column = udt.span().start().column;

            println!(
                "{:}:{:}:{:} - Missing comment for user-defined type definition",
                metadata.file_path, line, column
            )
        }
    }
}

impl Scanner for MissingComments {
    fn execute(&self, ast: &[Item], metadata: &Metadata) {
        for item in ast {
            match item {
                Item::Contract(contract) => {
                    self.check_missing_comments_for_contract(contract, metadata);
                    self.scan_in_contract(contract, metadata)
                }
                Item::Enum(enumeration) => {
                    self.check_missing_comments_for_enum(enumeration, metadata)
                }
                Item::Error(error) => self.check_missing_comments_for_error(error, metadata),
                Item::Function(function) => {
                    self.check_missing_comments_for_function(function, metadata)
                }
                Item::Struct(structure) => {
                    self.check_missing_comments_for_structure(structure, metadata)
                }
                Item::Udt(udt) => self.check_missing_comments_for_udt(udt, metadata),
                /* Events can not be declared at the root level. */
                /* Import directives don't have attributes. */
                /* Pragma directives don't have attributes. */
                /* Using declarations don't need to be commented. */
                /* Variable attributes don't include comments. */
                _ => {}
            }
        }
    }
}
