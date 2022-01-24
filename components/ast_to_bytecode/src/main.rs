use std::fs;

use bytecode::Inst;

fn main() {
    let insts = vec![
        Inst::I32Const(3),
        Inst::I32Const(4),
        Inst::I32Add,
        Inst::Print,
    ];
    let encoded = bincode::serialize(&insts).unwrap();
    fs::write("out.bin", encoded).unwrap();
}
