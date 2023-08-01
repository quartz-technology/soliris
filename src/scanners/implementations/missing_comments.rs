use syn_solidity::{
    Item, ItemContract, ItemEnum, ItemError, ItemEvent, ItemFunction, ItemStruct, ItemUdt,
};

use crate::scanners::{
    result::{Reporter, Severity},
    Scanner,
};

/// A scanner responsible for looking at various Solidity items and reporting if documentation (comments) is missing.
#[derive(Default)]
pub struct MissingComments {}

impl MissingComments {
    /// Scan every item in the provided contract object and looks for missing documentation.
    fn scan_in_contract(&self, contract: &ItemContract, reporter: &mut Reporter) {
        for item in &contract.body {
            match item {
                Item::Enum(enumeration) => {
                    self.check_missing_comments_for_enum(enumeration, reporter)
                }
                Item::Error(error) => self.check_missing_comments_for_error(error, reporter),
                Item::Event(event) => self.check_missing_comments_for_event(event, reporter),
                Item::Function(function) => {
                    self.check_missing_comments_for_function(function, reporter)
                }
                Item::Struct(structure) => {
                    self.check_missing_comments_for_structure(structure, reporter)
                }
                Item::Udt(udt) => self.check_missing_comments_for_udt(udt, reporter),
                /* Contracts can not be declared inside other contracts */
                /* Import directives don't have attributes. */
                /* Pragma directives don't have attributes. */
                /* Variable attributes don't include comments. */
                /* Using declarations don't need to be commented. */
                _ => {}
            }
        }
    }

    /// Report if a contract item is not documented.
    /// A potential fix should be:
    ///
    /// ```
    /// /**
    ///  * @dev Just a simple contract which does nothing!
    ///  */
    /// contract SimpleContract {}
    /// ```
    fn check_missing_comments_for_contract(
        &self,
        contract: &ItemContract,
        reporter: &mut Reporter,
    ) {
        if contract.attrs.is_empty() {
            let line = contract.span().start().line;
            let column = contract.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Warning,
                "Missing comment on contract definition",
            )
        }
    }

    /// Report if a enum is not documented.
    /// A potential fix should be:
    ///
    /// ```
    /// /**
    ///  * @dev Supported varieties of Quartz.
    ///  */
    /// enum QuartzType {
    ///     Amethyst,
    ///     Agate,
    /// }
    /// ```
    fn check_missing_comments_for_enum(&self, enumeration: &ItemEnum, reporter: &mut Reporter) {
        if enumeration.attrs.is_empty() {
            let line = enumeration.span().start().line;
            let column = enumeration.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Warning,
                "Missing comment on enum definition",
            )
        }
    }

    /// Report if a event is not documented.
    /// A potential fix should be:
    ///
    /// ```
    /// /// @dev Emitted when a new Quartz has been mined!
    /// event QuartzMined(QuartzType indexed variety);
    /// ```
    fn check_missing_comments_for_event(&self, event: &ItemEvent, reporter: &mut Reporter) {
        if event.attrs.is_empty() {
            let line = event.span().start().line;
            let column = event.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Warning,
                "Missing comment on event definition",
            )
        }
    }

    /// Report if an error is not documented.
    /// A potential fix should be:
    ///
    /// ```
    /// /// @dev Your favorite stone was broken :(
    /// error BrokenQuartz();
    /// ```
    fn check_missing_comments_for_error(&self, error: &ItemError, reporter: &mut Reporter) {
        if error.attrs.is_empty() {
            let line = error.span().start().line;
            let column = error.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Warning,
                "Missing comment on error definition",
            )
        }
    }

    /// Report if a function is not documented.
    /// A potential fix should be:
    ///
    /// ```
    /// /// @dev Tells you if the Amethyst variety is good looking.
    /// function IsAmethystBeautiful() public returns (bool) {
    ///    return true;
    /// }
    /// ```
    fn check_missing_comments_for_function(
        &self,
        function: &ItemFunction,
        reporter: &mut Reporter,
    ) {
        if function.attrs.is_empty() {
            let line = function.span().start().line;
            let column = function.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Warning,
                "Missing comment for function definition",
            )
        }
    }

    /// Report if a structure is not documented.
    /// A potential fix should be:
    ///
    /// ```
    /// /// @dev Composition of your favorite rock.
    /// structure Quartz {
    ///     QuartzType variety;
    /// }
    /// ```
    fn check_missing_comments_for_structure(
        &self,
        structure: &ItemStruct,
        reporter: &mut Reporter,
    ) {
        if structure.attrs.is_empty() {
            let line = structure.span().start().line;
            let column = structure.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Warning,
                "Missing comment for structure definition",
            )
        }
    }

    /// Report if a user-defined type is not documented.
    /// A potential fix should be:
    ///
    /// ```
    /// /// @dev Got this one from OpenZeppelin, I ran out of Quartz references.
    /// type ShortString is bytes32;
    /// ```
    fn check_missing_comments_for_udt(&self, udt: &ItemUdt, reporter: &mut Reporter) {
        if udt.attrs.is_empty() {
            let line = udt.span().start().line;
            let column = udt.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Warning,
                "Missing comment for user-defined type definition",
            )
        }
    }
}

impl Scanner for MissingComments {
    /// Scans every root item (recursively if there is a contract) and reports missing documentation.
    fn execute(&self, ast: &[Item], reporter: &mut Reporter) {
        for item in ast {
            match item {
                Item::Contract(contract) => {
                    self.check_missing_comments_for_contract(contract, reporter);
                    self.scan_in_contract(contract, reporter)
                }
                Item::Enum(enumeration) => {
                    self.check_missing_comments_for_enum(enumeration, reporter)
                }
                Item::Error(error) => self.check_missing_comments_for_error(error, reporter),
                Item::Function(function) => {
                    self.check_missing_comments_for_function(function, reporter)
                }
                Item::Struct(structure) => {
                    self.check_missing_comments_for_structure(structure, reporter)
                }
                Item::Udt(udt) => self.check_missing_comments_for_udt(udt, reporter),
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
