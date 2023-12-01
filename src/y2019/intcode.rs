enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
}

impl TryFrom<i32> for Operation {
    type Error = String;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(Self::Add),
            2 => Ok(Self::Multiply),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            99 => Ok(Self::Halt),
            _ => Err(format!("unknown opcode: {n}")),
        }
    }
}

enum ParameterMode {
    Position,
    Immediate,
}

impl TryFrom<i32> for ParameterMode {
    type Error = String;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(Self::Position),
            2 => Ok(Self::Immediate),
            _ => Err(format!("unknown parameter mode: {n}")),
        }
    }
}

#[derive(Default)]
pub struct IntCode {
    memory: Vec<i32>,
    ip: usize,
    halted: bool,
}

impl IntCode {
    pub fn new(memory: &[i32]) -> Self {
        Self { memory: memory.to_vec(), ip: 0, halted: false }
    }

    pub fn replace(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    fn arg(&self, pos: usize) -> i32 {
        let idx: usize =
            self.memory[self.ip + pos].try_into().expect("expect usize");

        self.memory[idx]
    }

    fn store(&mut self, pos: usize, v: i32) {
        let idx: usize =
            self.memory[self.ip + pos].try_into().expect("expect usize");

        self.memory[idx] = v;
    }

    pub fn dump(&self) -> &[i32] {
        &self.memory
    }

    fn step(&mut self) {
        let op = Operation::try_from(self.memory[self.ip])
            .expect("expected valid opcode");

        match op {
            Operation::Input => {}
            Operation::Output => {}
            Operation::Add => {
                let left = self.arg(1);
                let right = self.arg(2);

                self.store(3, left + right);
                self.ip += 4;
            }
            Operation::Multiply => {
                let left = self.arg(1);
                let right = self.arg(2);

                self.store(3, left * right);
                self.ip += 4;
            }
            Operation::Halt => self.halted = true,
        }
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let mut computer = IntCode::new(&input);

        let memory = vec![
            vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        ];

        for m in memory {
            computer.step();

            assert_eq!(computer.memory, m);
        }
    }
}
