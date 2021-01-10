use super::opcodes::*;
use super::types::*;

#[derive(Debug, Default)]
pub struct Expr {}

pub enum Instruction {
    Block { block_tyep: BlockType, insts: Vec<Box<Instruction>> },
    If { block_tyep: BlockType, insts1: Vec<Box<Instruction>>, insts2: Vec<Box<Instruction>> },
    Mem { opcode: Opcode, align: u32, offset: u32 },
    Br { label_idx: LabelIdx },
    BrIf { label_idx: LabelIdx },
    BrTable {tables: Vec<LabelIdx>, default_: LabelIdx }, 
}

enum BlockType {
    Empty,
    ValType(ValType),
    TypeIdx(u32),
}