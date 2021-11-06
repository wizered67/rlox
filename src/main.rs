mod chunk;
mod value;

fn main() {
    let mut chunk = chunk::Chunk::new();
    chunk.write(chunk::Opcode::Return as u8, 123);
    let constant = chunk.add_constant(1.2);
    chunk.write(chunk::Opcode::Constant as u8, 123);
    chunk.write(constant as u8, 123);
    chunk.write(chunk::Opcode::Return as u8, 123);
    chunk.disassemble("test chunk");
}
