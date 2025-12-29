use wasm_encoder::{BlockType, Instruction};





pub enum IrInstruction{
    I64Const(i64),
    I64Eqz,
    BrIf(u32),
    Br(u32),
    LocalSet(u32),
    LocalGet(u32),
    Call(u32),
    If(BlockType),
    Else,
    Block(BlockType),
    Loop(BlockType),
    Drop,
    Return,
    End,

    //arithmetic
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64Eq,
    I64LtS,
    I64GtS,
    I64ExtendI32S,
    I32Eqz,
}

impl IrInstruction{
    pub fn to_wasm(&self) -> Instruction<'_>{
        match self {
            IrInstruction::I64Const(v) => Instruction::I64Const(*v),
            IrInstruction::I64Eqz => Instruction::I64Eqz,
            IrInstruction::BrIf(idx) => Instruction::BrIf(*idx),
            IrInstruction::Br(idx) =>Instruction::Br(*idx),
            IrInstruction::LocalSet(idx) => Instruction::LocalSet(*idx),
            IrInstruction::LocalGet(idx) => Instruction::LocalGet(*idx),
            IrInstruction::Call(idx) => Instruction::Call(*idx),
            IrInstruction::If(block_type) => Instruction::If(*block_type),
            IrInstruction::Else => Instruction::Else,
            IrInstruction::Block(block_type) => Instruction::Block(*block_type),
            IrInstruction::Loop(block_type) => Instruction::Loop(*block_type),
            IrInstruction::Drop => Instruction::Drop,
            IrInstruction::Return => Instruction::Return,
            IrInstruction::End => Instruction::End,
            IrInstruction::I64Add => Instruction::I64Add,
            IrInstruction::I64Sub => Instruction::I64Sub,
            IrInstruction::I64Mul => Instruction::I64Mul,
            IrInstruction::I64DivS => Instruction::I64DivS,
            IrInstruction::I64Eq => Instruction::I64Eq,
            IrInstruction::I64LtS => Instruction::I64LtS,
            IrInstruction::I64GtS => Instruction::I64GtS,
            IrInstruction::I64ExtendI32S => Instruction::I64ExtendI32S,
            IrInstruction::I32Eqz => Instruction::I32Eqz,
        }
    }
}