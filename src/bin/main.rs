use genvm::{Vm, VmError, Runtime, RuntimeResult, Opcode};
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

struct TestError(&'static str);

impl VmError for TestError {
    fn stack_underflow() -> Self {
        TestError("stack underflow")
    }

    fn type_error() -> Self {
        TestError("type error")
    }
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Runtime for TestRuntime {
    type Value = TestValue;
    type Constant = TestConstant;
    type Error = TestError;

    fn constant(&mut self, constant: &Self::Constant) -> RuntimeResult<Self> {
        Ok(match constant {
            TestConstant::Int(i) => TestValue::Int(*i),
        })
    }

    fn add(&mut self, a: Self::Value, b: Self::Value) -> RuntimeResult<Self> {
        match (a, b) {
            (TestValue::Int(a), TestValue::Int(b)) => Ok(TestValue::Int(a + b)),
            _ => Err(Self::Error::type_error()),
        }
    }

    fn sub(&mut self, a: Self::Value, b: Self::Value) -> RuntimeResult<Self> {
        match (a, b) {
            (TestValue::Int(a), TestValue::Int(b)) => Ok(TestValue::Int(a - b)),
            _ => Err(Self::Error::type_error()),
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
