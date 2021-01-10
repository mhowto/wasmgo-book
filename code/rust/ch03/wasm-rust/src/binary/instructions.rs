use super::opcodes::*;
use super::types::*;

#[derive(Debug, Default)]
pub struct Expr {}

pub trait Instruction {
    fn get_opcode(&self) -> Opcode;
    fn get_opname(&self) -> &str {
        OPNAMES[self.get_opcode() as usize]
    }
}

enum BlockType {
    Empty,
    ValType(ValType),
    TypeIdx(u32),
}

pub struct BlockInst {
    block_type: BlockType,
    insts: Vec<Box<Instruction>>,
} 

impl Instruction for BlockInst {
    fn get_opcode(&self) -> Opcode {
        Opcode::Block
    } 
}