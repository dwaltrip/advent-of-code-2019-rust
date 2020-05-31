// ---------------------------------------------------
// Copied and modified from Day 7 intcode_computer.rs
// ---------------------------------------------------

const DEBUG: bool = false;
// const DEBUG: bool = true;

const MAX_LOOP_ITERATIONS: isize = 10_000;

pub struct Program {
  values: Vec<isize>,
  status: ProgramStatus,
  instruction_pointer: usize,
  relative_base: isize,
}

pub enum ProgramStatus {
  Running,
  Halted,
}

impl Program {
  pub fn new(values: &[isize]) -> Program {
    // Day 9 instructions say that "The computer's available memory should be
    //   much larger than the initial program".
    // Guesstimating, I chose 100x the program length to meet this requirement.
    let mut values_with_extra_memory = values.to_vec();
    values_with_extra_memory.resize(values.len() * 100, 0);
    Program {
      values: values_with_extra_memory,
      status: ProgramStatus::Running,
      instruction_pointer: 0,
      relative_base: 0,
    }
  }

  pub fn is_halted(&self) -> bool {
    match self.status {
      ProgramStatus::Halted => true,
      _ => false,
    }
  }

  pub fn run(&mut self, inputs: &[isize]) -> Vec<isize> {
    if self.is_halted() {
      panic!("Cant run a halted program");
    }

    let mut inputs_iter = inputs.iter();
    let mut output = Vec::new();
    let mut iteration_count = 0;

    if DEBUG {
      println!("");
      println!("-----------------------------");
      println!("Inputs: {:?}", inputs);
      println!("Initial program: {:?}", self.values);
    }

    loop {
      let opcode_data = self.next_opcode_data();

      let opcode_num: isize =
        match opcode_data.len() {
          0 => panic!("Invalid opcode_data {:?}", opcode_data),
          1 => opcode_data[..].parse(),
          _ => opcode_data[opcode_data.len() - 2..].parse()
        }
        .expect(&format!("Could not parse opcode_num: {:?}", opcode_data));

      let opcode = match opcode_num {
        1 => Opcode::Add,
        2 => Opcode::Multiply,
        3 => Opcode::Input,
        4 => Opcode::Output,
        5 => Opcode::JumpIfTrue,
        6 => Opcode::JumpIfFalse,
        7 => Opcode::LessThan,
        8 => Opcode::Equals,
        9 => Opcode::RelativeBaseOffset,
        99 => Opcode::Halt,
        _ => panic!("Invalid opcode: {:?}", opcode_num),
      };
      let num_params = match opcode {
        Opcode::Add => 3,
        Opcode::Multiply => 3,
        Opcode::Input => 1,
        Opcode::Output => 1,
        Opcode::JumpIfTrue => 2,
        Opcode::JumpIfFalse => 2,
        Opcode::LessThan => 3,
        Opcode::Equals => 3,
        Opcode::RelativeBaseOffset => 1,
        Opcode::Halt => 0,
      };

      let mut modes: Vec<isize> = 
        if opcode_data.len() > 1 {
          opcode_data[..opcode_data.len() - 2]
            .chars()
            .map(|c| c.to_digit(10).expect(&format!("{:?} is not a digit", c)) as isize)
            .rev()
            .collect()
        } else {
          vec![]
        };

      while modes.len() < num_params {
        modes.push(0);
      }

      let params: Vec<Parameter> = (0..num_params)
        .map(|i| Parameter::new(
          self.values[self.instruction_pointer + (i+1)],
          modes[i],
        ))
        .collect();

      let mut should_increment_pointer = true;

      if DEBUG {
        println!("Doing opcode {:?}", opcode);
        println!("\tparams: {:?}", params);
      }

      match opcode {
        Opcode::Add => {
          self.write_value(
            &params[2],
            self.get_param_val(&params[0]) + self.get_param_val(&params[1]),
          );
        },
        Opcode::Multiply => {
          self.write_value(
            &params[2],
            self.get_param_val(&params[0]) * self.get_param_val(&params[1]),
          );
        },
        Opcode::Input => {
          match inputs_iter.next() {
            Some(&value) => self.write_value(&params[0], value),
            None => break,
          }
        },
        Opcode::Output => {
          output.push(self.get_param_val(&params[0]));
        },
        Opcode::JumpIfTrue => {
          if self.get_param_val(&params[0]) != 0 {
            should_increment_pointer = false;
            self.instruction_pointer = self.get_param_val(&params[1]) as usize;
          }
        },
        Opcode::JumpIfFalse => {
          if self.get_param_val(&params[0]) == 0 {
            should_increment_pointer = false;
            self.instruction_pointer = self.get_param_val(&params[1]) as usize;
          }
        },
        Opcode::LessThan => {
          let is_less_than =
            self.get_param_val(&params[0]) <
            self.get_param_val(&params[1]);
          self.write_value(&params[2], if is_less_than { 1 } else { 0 });
        },
        Opcode::Equals => {
          let is_equal =
            self.get_param_val(&params[0]) ==
            self.get_param_val(&params[1]);
          self.write_value(&params[2], if is_equal { 1 } else { 0 });
        },
        Opcode::RelativeBaseOffset => {
          self.relative_base += self.get_param_val(&params[0]);
        },
        Opcode::Halt => {
          self.status = ProgramStatus::Halted;
          break
        }
      }

      if should_increment_pointer {
        self.instruction_pointer += num_params + 1;
      }

      if DEBUG {
        println!("\tUpdated state: {:?}", self.values);
        println!("\tinstruction_pointer: {:?}", self.instruction_pointer);
      }

      iteration_count += 1;
      if iteration_count >= MAX_LOOP_ITERATIONS {
        panic!("MAX_LOOP_ITERATIONS exceeded. Aborting... ");
      }

      if self.instruction_pointer >= self.values.len() {
        println!("Unexpected... instruction_pointer is too large.");
        break
      }
    }

    output
  }

  fn next_opcode_data(&self) -> String {
    self.values[self.instruction_pointer].to_string()
  }

  fn get_param_val(&self, param: &Parameter) -> isize {
    match param.mode {
      ParameterMode::Immediate => param.value,
      _ => self.values[self.get_index_for_param(&param)],
    }
  }

  fn write_value(&mut self, write_param: &Parameter, value: isize) {
    let index_to_write_to = match write_param.mode {
      ParameterMode::Immediate => {
        // Per Day 5 instructions
        panic!("Params that an instruction writes to will never be in immediate mode.");
      },
      _ => self.get_index_for_param(&write_param),
    };
    self.values[index_to_write_to as usize] = value;
  }

  fn get_index_for_param(&self, param: &Parameter) -> usize {
    (match param.mode {
      ParameterMode::Position => param.value,
      ParameterMode::Immediate => {
        panic!("Immediate mode params do not correspond to indices");
      },
      ParameterMode::Relative => self.relative_base + param.value,
    }) as usize
  }
}


#[derive(Debug)]
struct Parameter {
  value: isize,
  mode: ParameterMode,
}

impl Parameter {
  fn new(value: isize, mode: isize) -> Self {
    let mode = match mode {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate,
      2 => ParameterMode::Relative,
      _ => panic!("Invalid mode: {:?}", mode),
    };
    Self { value, mode }
  }
}

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate,
  Relative,
}

#[derive(Debug)]
enum Opcode {
  Add,
  Multiply,
  Input,
  Output,
  JumpIfTrue,
  JumpIfFalse,
  LessThan,
  Equals,
  RelativeBaseOffset,
  Halt,
}


#[cfg(test)]
mod tests {
  use super::*;

  struct TestCase {
    program: Vec<isize>,
    inputs: Vec<isize>,
    end_state: Vec<isize>,
    output: Vec<isize>,
  }

  fn run_test_cases(cases: &Vec<TestCase>) {
    for case in cases {
      let mut program = Program::new(&case.program);
      let output = program.run(&case.inputs);
      assert!(case.end_state.len() > 0);
      assert_eq!(
        &program.values[..case.end_state.len()],
        &case.end_state[..],
      );
      assert_eq!(output, case.output);
    }
  }

  #[test]
  fn works_with_op_codes_1_2_99() {
    // Example programs from Day 2
    let cases = vec![
      TestCase {
        program:   vec![1,0,0,0,99],
        end_state: vec![2,0,0,0,99],
        inputs: vec![],
        output: vec![],
      },
      TestCase {
        program:   vec![2,3,0,3,99],
        end_state: vec![2,3,0,6,99],
        inputs: vec![],
        output: vec![],
      },
      TestCase {
        program:   vec![2,4,4,5,99,0],
        end_state: vec![2,4,4,5,99,9801],
        inputs: vec![],
        output: vec![],
      },
      TestCase {
        program:   vec![1,1,1,4,99,5,6,0,99],
        end_state: vec![30,1,1,4,2,5,6,0,99],
        inputs: vec![],
        output: vec![],
      },
      TestCase {
        program:   vec![1,9,10,3,2,3,11,0,99,30,40,50],
        end_state: vec![3500,9,10,70,2,3,11,0,99,30,40,50],
        inputs: vec![],
        output: vec![],
      },
    ];

    run_test_cases(&cases); 
  }

  #[test]
  fn op_codes_3_and_4() {
    let cases = vec![
      TestCase {
        program: vec![3,0,4,0,99],
        inputs: vec![1],
        end_state: vec![1, 0, 4, 0, 99],
        output: vec![1],
      },
    ];

    run_test_cases(&cases);
  }

  #[test]
  fn op_code_5() {
    // Basic tests for jump-if-true. I wrote these.
    let cases = vec![
      TestCase {
        // Address 8 is non-zero, so it does jump.
        program: vec![1005, 8, 5, 4, 9, 4, 10, 99, 1, -1, -2],
        inputs: vec![-1],
        end_state: vec![1005, 8, 5, 4, 9, 4, 10, 99, 1, -1, -2],
        output: vec![-2],
      },
      TestCase {
        // Address 8 is 0, so it does NOT jump.
        program: vec![1005, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
        inputs: vec![-1],
        end_state: vec![1005, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
        output: vec![-1, -2],
      },
    ];

    run_test_cases(&cases);
  }

  #[test]
  fn op_code_6() {
    // Basic tests for jump-if-false. I wrote these.
    let cases = vec![
      TestCase {
        // Address 8 is non-zero, so it does NOT jump.
        program: vec![1006, 8, 5, 4, 9, 4, 10, 99, 1, -1, -2],
        inputs: vec![-1],
        end_state: vec![1006, 8, 5, 4, 9, 4, 10, 99, 1, -1, -2],
        output: vec![-1, -2],
      },
      TestCase {
        // Address 8 is 0, so it does jump.
        program: vec![1006, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
        inputs: vec![-1],
        end_state: vec![1006, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
        output: vec![-2],
      },
    ];

    run_test_cases(&cases);
  }

  #[test]
  fn op_code_8() {
    let program1 = vec![3,9,8,9,10,9,4,9,99,-1,8];
    let program2 = vec![3,3,1108,-1,8,3,4,3,99];

    let cases = vec![
      // position mode (1st bulleted example in part 2)
      TestCase {
        program: program1.clone(),
        inputs: vec![8], // equals 8
        end_state: vec![3,9,8,9,10,9,4,9,99,1,8],
        output: vec![1],
      },
      TestCase {
        program: program1.clone(),
        inputs: vec![5], // less than 8
        end_state: vec![3,9,8,9,10,9,4,9,99,0,8],
        output: vec![0],
      },
      TestCase {
        program: program1.clone(),
        inputs: vec![900], // greater than 8
        end_state: vec![3,9,8,9,10,9,4,9,99,0,8],
        output: vec![0],
      },

      // immediate mode (3rd bulleted example in part 2)
      TestCase {
        program: program2.clone(),
        inputs: vec![8], // equals 8,
        end_state: vec![3,3,1108,1,8,3,4,3,99],
        output: vec![1],
      },
      TestCase {
        program: program2.clone(),
        inputs: vec![5], // less than 8,
        end_state: vec![3,3,1108,0,8,3,4,3,99],
        output: vec![0],
      },
      TestCase {
        program: program2.clone(),
        inputs: vec![900], // greater than 8,
        end_state: vec![3,3,1108,0,8,3,4,3,99],
        output: vec![0],
      },
    ];

    run_test_cases(&cases);
  }

  #[test]
  fn op_code_9() {
    let quine_program = vec![
      109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99,
    ];

    let cases = vec![
      TestCase {
        program: quine_program.clone(),
        inputs: vec![],
        end_state: quine_program.clone(),
        output: quine_program.clone(),
      },
    ];

    run_test_cases(&cases);
  }
}
