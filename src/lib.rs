pub trait Runtime {
    type Value: Default;
    type Constant;
    type Error: VmError;

    fn add(&mut self, a: Self::Value, b: Self::Value) -> Result<Self::Value, Self::Error>;
    fn sub(&mut self, a: Self::Value, b: Self::Value) -> Result<Self::Value, Self::Error>;
    fn constant(&mut self, constant: &Self::Constant) -> Result<Self::Value, Self::Error>;
}

#[repr(u8)]
pub enum Opcode {
    Halt = 0,
    Add,
    Sub,
    Constant,
}

pub trait VmError {
    fn stack_underflow() -> Self;
}

pub struct Vm<'a, T: Runtime> {
    runtime: T,
    bytecode: &'a [u8],
    stack: Vec<T::Value>,
    constant_table: &'a [T::Constant],
    ip: usize,
    sp: usize,
}

impl<'a, T: Runtime> Vm<'a, T> {
    pub fn new(runtime: T, bytecode: &'a [u8], constant_table: &'a [T::Constant]) -> Self {
        Self {
            runtime,
            bytecode,
            constant_table,
            stack: Vec::new(),
            ip: 0,
            sp: 0,
        }
    }

    fn pop(&mut self) -> Result<T::Value, T::Error> {
        self.stack.pop().ok_or_else(T::Error::stack_underflow)
    }

    fn push(&mut self, value: T::Value) {
        self.stack.push(value);
        self.sp += 1;
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.bytecode[self.ip];
        self.ip += 1;
        byte
    }

    fn next_instruction(&mut self) -> Opcode {
        unsafe { std::mem::transmute(self.next_byte()) }
    }

    pub fn run(&mut self) -> Result<T::Value, T::Error> {
        loop {
            match self.next_instruction() {
                Opcode::Halt => break,
                Opcode::Add => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    let sum = self.runtime.add(a, b)?;
                    self.push(sum);
                }
                Opcode::Sub => {
                    let a = self.pop()?;
                    let b = self.pop()?;
                    let diff = self.runtime.sub(a, b)?;
                    self.push(diff);
                }
                Opcode::Constant => {
                    let idx = self.next_byte();
                    let constant = &self.constant_table[idx as usize];
                    let value = self.runtime.constant(constant)?;
                    self.push(value);
                }
            }
        }

        Ok(self.stack.pop().unwrap_or_default())
    }
}
