use wasm_encoder::{BlockType, Instruction};

/// Compiler-internal instruction set that maps 1:1 to wasm opcodes.
pub enum IrInstruction {
    I64Const(i64),
    I32Const(i32),
    F64Const(f64),
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

    // Arithmetic
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,

    I64Eq,
    I64LtS,
    I64GtS,
    I32Eq,
    F64Eq,
    F64Lt,
    F64Gt,

    I32Eqz,
}

impl IrInstruction {
    /// Converts this IR instruction into a wasm-encoder instruction.
    pub fn to_wasm(&self) -> Instruction<'_> {
        match self {
            IrInstruction::I64Const(v) => Instruction::I64Const(*v),
            IrInstruction::I32Const(v) => Instruction::I32Const(*v),
            IrInstruction::F64Const(v) => Instruction::F64Const((*v).into()),
            IrInstruction::I64Eqz => Instruction::I64Eqz,
            IrInstruction::BrIf(idx) => Instruction::BrIf(*idx),
            IrInstruction::Br(idx) => Instruction::Br(*idx),
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
            IrInstruction::F64Add => Instruction::F64Add,
            IrInstruction::F64Sub => Instruction::F64Sub,
            IrInstruction::F64Mul => Instruction::F64Mul,
            IrInstruction::F64Div => Instruction::F64Div,
            IrInstruction::I64Eq => Instruction::I64Eq,
            IrInstruction::I64LtS => Instruction::I64LtS,
            IrInstruction::I64GtS => Instruction::I64GtS,
            IrInstruction::I32Eq => Instruction::I32Eq,
            IrInstruction::F64Eq => Instruction::F64Eq,
            IrInstruction::F64Lt => Instruction::F64Lt,
            IrInstruction::F64Gt => Instruction::F64Gt,
            IrInstruction::I32Eqz => Instruction::I32Eqz,
        }
    }
}
