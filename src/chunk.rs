use crate::value;
use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum Opcode {
  Constant,
  Return,
}

impl Opcode {
  pub fn from_u8(n: u8) -> Option<Opcode> {
    return num::FromPrimitive::from_u8(n);
  }
}

pub struct Chunk {
  code: Vec<u8>,
  constants: Vec<value::Value>,
  lines: Vec<i32>,
}

impl Chunk {
  pub fn new() -> Chunk {
    return Chunk {
      code: Vec::new(),
      constants: Vec::new(),
      lines: Vec::new(),
    };
  }
  pub fn write(&mut self, byte: u8, line: i32) {
    self.code.push(byte);
    self.lines.push(line);
  }
  pub fn add_constant(&mut self, value: value::Value) -> usize {
    self.constants.push(value);
    return self.constants.len() - 1;
  }
  pub fn disassemble(&self, name: &str) {
    println!("== {} ==", name);
    let mut offset = 0;
    while offset < self.code.len() {
      offset = self.disassemble_instruction(offset);
    }
  }
  fn disassemble_instruction(&self, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
      print!("   | ");
    } else {
      print!("{:4} ", self.lines[offset]);
    }

    let raw_value = self.code[offset];
    let instruction = Opcode::from_u8(raw_value);
    match instruction {
      Some(Opcode::Return) => {
        return simple_instruction("OP_RETURN", offset);
      }
      Some(Opcode::Constant) => {
        return self.constant_instruction("OP_CONSTANT", offset);
      }
      None => {
        println!("Unknown opcode {}", raw_value);
        return offset + 1;
      }
    }
  }

  fn constant_instruction(&self, name: &str, offset: usize) -> usize {
    let constant = self.code[offset + 1] as usize;
    print!("{:16} {:4} ", name, constant);
    value::print_value(self.constants[constant]);
    println!();
    return offset + 2;
  }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
  println!("{}", name);
  return offset + 1;
}
