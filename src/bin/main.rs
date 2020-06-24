use genvm::{Vm, VmError, Runtime, Opcode};
use std::fmt;

struct TestRuntime;

#[derive(Debug)]
enum TestValue {
    Null,
    Int(i64),
}

impl Default for TestValue {
    fn default() -> Self {
        Self::Null
    }
}

enum TestConstant {
    Int(i64),
}

enum TestError {
    StackUnderflow,
    TypeError,
}

impl VmError for TestError {
    fn stack_underflow() -> Self {
        TestError::StackUnderflow
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::StackUnderflow => "stack underflow",
            Self::TypeError => "type error",
        })
    }
}

impl Runtime for TestRuntime {
    type Value = TestValue;
    type Constant = TestConstant;
    type Error = TestError;

    fn constant(&mut self, constant: &Self::Constant) -> Result<Self::Value, Self::Error> {
        Ok(match constant {
            TestConstant::Int(i) => TestValue::Int(*i),
        })
    }

    fn add(&mut self, a: Self::Value, b: Self::Value) -> Result<Self::Value, Self::Error> {
        match (a, b) {
            (TestValue::Int(a), TestValue::Int(b)) => Ok(TestValue::Int(a + b)),
            _ => Err(Self::Error::TypeError),
        }
    }

    fn sub(&mut self, a: Self::Value, b: Self::Value) -> Result<Self::Value, Self::Error> {
        match (a, b) {
            (TestValue::Int(a), TestValue::Int(b)) => Ok(TestValue::Int(a - b)),
            _ => Err(Self::Error::TypeError),
        }
    }
}

fn main() {
    let runtime = TestRuntime;
    let constant_table = &[TestConstant::Int(5), TestConstant::Int(8)];
    let mut vm = Vm::new(
        runtime,
        &[
            Opcode::Constant as _,
            0,
            Opcode::Constant as _,
            1,
            Opcode::Add as _,
            Opcode::Halt as _,
        ],
        constant_table,
    );

    match vm.run() {
        Ok(value) => println!("Got {:?}", value),
        Err(e) => eprintln!("Failed: {}", e),
    }
}
