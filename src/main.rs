mod chunk;
mod value;
mod vm;

use chunk::Opcode::*;

fn main() {
    let mut vm = vm::VM::new();

    let mut chunk = chunk::Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(Constant as u8, 123);
    chunk.write(constant as u8, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(Constant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(Add as u8, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(Constant as u8, 123);
    chunk.write(constant as u8, 123);

    chunk.write(Divide as u8, 123);
    chunk.write(Negate as u8, 123);

    chunk.write(Return as u8, 123);
    // chunk.disassemble("test chunk");
    vm.interpret(chunk);
}
