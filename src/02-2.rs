use std::fs;
use std::convert::TryFrom;
use anyhow::Result as AnyResult;
use thiserror::Error;

#[derive(Debug, Error)]
enum MyError {
    #[error("encountered unknown opcode `{0}`")]
    UnknownOpCode(i64),
    #[error("index out of bounds `{0}`")]
    OutOfBounds(usize),
}

#[derive(Debug)]
enum OpCode {
    Halt,
    Add,
    Mul,
}

impl TryFrom<&i64> for OpCode {
    type Error = MyError;
    fn try_from(i: &i64) -> Result<Self, Self::Error> {
        match *i {
            99 => Ok(OpCode::Halt),
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            _ => Err(MyError::UnknownOpCode(*i))
        }
    }
}

trait IOperation {
    fn is_halt(&self) -> bool {
        false
    }
    fn execute(self, memory: &mut [i64]);
}

#[derive(Debug)]
struct AddOperation {
    src1: usize,
    src2: usize,
    dest: usize,
}

impl IOperation for AddOperation {
    fn execute(self, memory: &mut [i64]) {
        memory[self.dest] = memory[self.src1] + memory[self.src2];
    }
}

#[derive(Debug)]
struct MulOperation {
    src1: usize,
    src2: usize,
    dest: usize,
}

impl IOperation for MulOperation {
    fn execute(self, memory: &mut [i64]) {
        memory[self.dest] = memory[self.src1] * memory[self.src2];
    }
}

#[derive(Debug)]
struct HaltOperation;

impl IOperation for HaltOperation {
    fn is_halt(&self) -> bool {
        true
    }
    fn execute(self, _: &mut [i64]) {
        panic!("Can't execute Halt")
    }
}

#[derive(Debug)]
enum Operation {
    Halt(HaltOperation),
    Add(AddOperation),
    Mul(MulOperation),
}

impl IOperation for Operation {
    fn is_halt(&self) -> bool {
        match self {
            Self::Halt(ref inner) => inner.is_halt(),
            Self::Add(ref inner) => inner.is_halt(),
            Self::Mul(ref inner) => inner.is_halt(),
        }
    }
    fn execute(self, memory: &mut [i64]) {
        match self {
            Self::Halt(inner) => inner.execute(memory),
            Self::Add(inner) => inner.execute(memory),
            Self::Mul(inner) => inner.execute(memory),
        }
    }
}

fn op_at(memory: &[i64], index: usize) -> Result<Operation, MyError> {
    let opcode = OpCode::try_from(
        memory.get(index).ok_or(MyError::OutOfBounds(index))?)?;
    Ok(match opcode {
        OpCode::Halt => Operation::Halt(HaltOperation),
        OpCode::Add => Operation::Add(AddOperation {
            src1: *memory.get(index + 1).ok_or(MyError::OutOfBounds(index + 1))? as _,
            src2: *memory.get(index + 2).ok_or(MyError::OutOfBounds(index + 2))? as _,
            dest: *memory.get(index + 3).ok_or(MyError::OutOfBounds(index + 3))? as _,
        }),
        OpCode::Mul => Operation::Mul(MulOperation {
            src1: *memory.get(index + 1).ok_or(MyError::OutOfBounds(index + 1))? as _,
            src2: *memory.get(index + 2).ok_or(MyError::OutOfBounds(index + 2))? as _,
            dest: *memory.get(index + 3).ok_or(MyError::OutOfBounds(index + 3))? as _,
        })
    })
}

fn run(mut memory: Vec<i64>, at1: i64, at2: i64) -> Result<i64, MyError> {
    memory[1] = at1;
    memory[2] = at2;
    let mut i = 0;
    loop {
        let op = op_at(&memory, i)?;
        if op.is_halt() {
            break;
        }
        op.execute(&mut memory);
        i += 4;
    }
    Ok(memory[0])
}

fn main() -> AnyResult<()> {
    let memory: Vec<i64> = fs::read_to_string("input/02")?
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    let expected = 19690720;
    let mut answer = None;
    for at1 in 0..100 {
        for at2 in 0..100 {
            if run(memory.clone(), at1, at2).ok() == Some(expected) {
                answer = Some(at1 * 100 + at2);
            }
        }
    }
    println!("{:?}", answer);
    Ok(())
}