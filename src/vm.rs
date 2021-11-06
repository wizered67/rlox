use crate::chunk::{Chunk, Opcode};
use crate::value;

const DEBUG_TRACE_EXECUTION: bool = true;

macro_rules! binary_op {
  ($self: ident, $op: tt) => {
    let b = $self.pop();
    let a = $self.pop();
    $self.push(a $op b);
  };
}

pub struct VM {
  chunk: Chunk,
  ip: usize,
  stack: Vec<value::Value>,
}

impl VM {
  pub fn new() -> VM {
    return VM {
      chunk: Chunk::new(),
      ip: 0,
      stack: Vec::new(),
    };
  }

  fn push(&mut self, value: value::Value) {
    self.stack.push(value);
  }

  fn pop(&mut self) -> value::Value {
    // TODO: figure out if we want any special handling for
    // when there's nothing to pop off.
    return self.stack.pop().unwrap();
  }

  pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
    self.chunk = chunk;
    self.ip = 0;
    return self.run();
  }

  fn run(&mut self) -> InterpretResult {
    loop {
      if DEBUG_TRACE_EXECUTION {
        print!("          ");
        for value in self.stack.iter() {
          print!("[ ");
          value::print_value(*value);
          print!(" ]");
        }
        println!();
        self.chunk.disassemble_instruction(self.ip);
      }

      let instruction = self.chunk.code[self.ip];
      self.ip += 1;
      let opcode = Opcode::from_u8(instruction).unwrap();

      match opcode {
        Opcode::Return => {
          value::print_value(self.pop());
          println!();
          return InterpretResult::Ok;
        }
        Opcode::Constant => {
          let constant = self.read_constant();
          self.push(constant);
        }
        Opcode::Add => {
          binary_op!(self, +);
        }
        Opcode::Subtract => {
          binary_op!(self, -);
        }
        Opcode::Multiply => {
          binary_op!(self, *);
        }
        Opcode::Divide => {
          binary_op!(self, /);
        }
        Opcode::Negate => {
          let value = self.pop();
          self.push(-value);
        }
      }
    }
  }

  fn read_constant(&mut self) -> value::Value {
    let byte = self.chunk.code[self.ip];
    self.ip += 1;
    return self.chunk.constants[byte as usize];
  }
}

pub enum InterpretResult {
  Ok,
  CompileError,
  RuntimeError,
}
