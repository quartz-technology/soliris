use syn_solidity::{Item, ItemContract, ItemStruct, Spanned, VariableDeclaration};

use crate::scanners::{
    result::{Reporter, Severity},
    Scanner,
};

const BIN_SIZE: u16 = 256;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct BinItem {
    name: String,
    size: u16,
}

impl From<&VariableDeclaration> for BinItem {
    fn from(field: &VariableDeclaration) -> Self {
        BinItem {
            name: field.name.as_ref().unwrap().as_string(),
            size: match field.ty {
                syn_solidity::Type::Address(_, _) => 20,
                syn_solidity::Type::Bool(_) => 1,
                syn_solidity::Type::String(_) => 32,
                syn_solidity::Type::Bytes(_) => 32,
                syn_solidity::Type::FixedBytes(_, s) => s.get(),
                syn_solidity::Type::Int(_, s) => s.unwrap().get(),
                syn_solidity::Type::Uint(_, s) => s.unwrap().get(),
                syn_solidity::Type::Array(_) => 32,
                syn_solidity::Type::Tuple(_) => todo!(),
                syn_solidity::Type::Function(_) => todo!(),
                syn_solidity::Type::Mapping(_) => 32,
                syn_solidity::Type::Custom(_) => todo!(),
            },
        }
    }
}

trait RepackStrategy {
    fn repack(&self, initial_structure: &[BinItem]) -> Vec<BinItem>;
}

#[derive(Default)]
struct FirstFitDecreasing {}

impl RepackStrategy for FirstFitDecreasing {
    fn repack(&self, initial_structure: &[BinItem]) -> Vec<BinItem> {
        let mut cpy = initial_structure.to_vec();

        // Reorder structure fields from biggest size to smallest.
        cpy.sort_by(|a, b| b.size.cmp(&a.size));

        cpy
    }
}

#[derive(Default)]
pub struct StructRepacker {}

impl StructRepacker {
    fn scan_contract(&self, contract: &ItemContract, reporter: &mut Reporter) {
        for item in &contract.body {
            if let Item::Struct(structure) = item {
                self.scan_struct(structure, reporter)
            }
        }
    }

    fn scan_struct(&self, structure: &ItemStruct, reporter: &mut Reporter) {
        let initial_structure: Vec<BinItem> = structure.fields.iter().map(BinItem::from).collect();
        let initial_slot_consumption = self.compute_actual_slot_consumption(&initial_structure);
        let optimum_slot_consumption = self.compute_optimum_slot_consumption(&initial_structure);

        if initial_slot_consumption != optimum_slot_consumption {
            let strategy = FirstFitDecreasing::default();
            let _repacked_struct = strategy.repack(&initial_structure);

            let line = structure.span().start().line;
            let column = structure.span().start().column;

            reporter.report(
                line,
                column,
                Severity::Boost,
                "Structure can be re-written to potentially consume less storage slots",
            )
        }
    }

    fn compute_optimum_slot_consumption(&self, structure: &[BinItem]) -> u16 {
        (structure.iter().map(|f| f.size).sum::<u16>() as f64 / BIN_SIZE as f64) as u16
    }

    fn compute_actual_slot_consumption(&self, structure: &Vec<BinItem>) -> u16 {
        let mut slot_consumption: u16 = 0;
        let mut carry: u16 = 0;

        for field in structure {
            match carry + field.size {
                slot if slot < BIN_SIZE => {
                    carry += field.size;
                }
                slot if slot == BIN_SIZE => {
                    slot_consumption += 1;
                    carry = 0;
                }
                _ => {
                    slot_consumption += 1;
                    carry = field.size;
                }
            }
        }

        if carry != 0 {
            slot_consumption += 1
        }

        slot_consumption
    }
}

impl Scanner for StructRepacker {
    fn execute(&self, ast: &[Item], reporter: &mut Reporter) {
        for item in ast {
            match item {
                Item::Contract(contract) => self.scan_contract(contract, reporter),
                Item::Struct(structure) => self.scan_struct(structure, reporter),
                _ => {}
            }
        }
    }
}
