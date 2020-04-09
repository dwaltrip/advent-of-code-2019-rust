use std::fs;

const DEBUG: bool = false;
// const DEBUG: bool = true;

fn main() {
  let raw_input = fs::read_to_string("./puzzle-input.txt")
    .expect("Something went wrong reading the file");

  let program: Vec<isize> = raw_input
    .trim()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();

  solve_part_1(&program);
  solve_part_2(&program);
}

fn solve_part_1(program: &Vec<isize>) {
  println!("---- Solving part 1! ----");
  let output = run_intcode_program(&mut program.clone(), 1);
  println!("Output: {:?}", output);
}

fn solve_part_2(program: &Vec<isize>) {
  println!("---- Solving part 2! ----");
  let output = run_intcode_program(&mut program.clone(), 5);
  println!("Output: {:?}", output);
}

const MAX_LOOP_ITERATIONS: isize = 10_000;

fn run_intcode_program(values: &mut Vec<isize>, input: isize) -> Vec<isize> {
  let mut output = Vec::new();

  let mut instruction_pointer = 0;
  let mut iteration_count = 0;

  if DEBUG {
    println!("");
    println!("-----------------------------");
    println!("Input: {:?}", input);
    println!("Initial program: {:?}", values);
  }

  loop {
    let opcode_data = values[instruction_pointer].to_string();

    let opcode_num: isize =
      match opcode_data.len() {
        0 => panic!("Invalid opcode_data {:?}", opcode_data),
        1 => opcode_data[..].parse(),
        _ => opcode_data[opcode_data.len() - 2..].parse()
      }
      .expect(&format!("Could not parse opcode_num: {:?}", opcode_data));

    let mut modes: Vec<isize> = 
      if opcode_data.len() > 1 {
        opcode_data[..opcode_data.len() - 2]
          .chars()
          .map(|c| c.to_digit(10).expect(&format!("{:?} is not a digit", c)) as isize)
          .rev()
          .collect()
      } else {
        vec![]
      }
    ;

    let opcode = match opcode_num {
      1 => Opcode::Add,
      2 => Opcode::Multiply,
      3 => Opcode::Input,
      4 => Opcode::Output,
      5 => Opcode::JumpIfTrue,
      6 => Opcode::JumpIfFalse,
      7 => Opcode::LessThan,
      8 => Opcode::Equals,
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
      Opcode::Halt => 0,
    };

    while modes.len() < num_params {
      modes.push(0);
    }

    let mut params = Vec::new();

    for i in 0..num_params {
      params.push(Parameter {
        value: values[instruction_pointer + (i+1)],
        mode: match modes[i] {
          0 => ParameterMode::Position,
          1 => ParameterMode::Immediate,
          _ => panic!("Invalid mode: {:?}", modes[i]),
        },
      });
    }

    let mut should_increment_pointer = true;

    if DEBUG {
      println!("Doing opcode {:?}", opcode);
      println!("\tparams: {:?}", params);
    }

    match opcode {
      Opcode::Add => {
        values[params[2].value as usize] =
          get_param_val(&values, &params[0]) +
          get_param_val(&values, &params[1]);
      },
      Opcode::Multiply => {
        values[params[2].value as usize] =
          get_param_val(&values, &params[0]) *
          get_param_val(&values, &params[1]);
      },
      Opcode::Input => {
        values[params[0].value as usize] = input;
      },
      Opcode::Output => {
        output.push(get_param_val(&values, &params[0]));
      },
      Opcode::JumpIfTrue => {
        if get_param_val(&values, &params[0]) != 0 {
          should_increment_pointer = false;
          instruction_pointer = get_param_val(&values, &params[1]) as usize;
        }
      },
      Opcode::JumpIfFalse => {
        if get_param_val(&values, &params[0]) == 0 {
          should_increment_pointer = false;
          instruction_pointer = get_param_val(&values, &params[1]) as usize;
        }
      },
      Opcode::LessThan => {
        let is_less_than =
          get_param_val(&values, &params[0]) <
          get_param_val(&values, &params[1]);

        values[params[2].value as usize] = if is_less_than { 1 } else { 0 };
      },
      Opcode::Equals => {
        let is_equal =
          get_param_val(&values, &params[0]) ==
          get_param_val(&values, &params[1]);

        values[params[2].value as usize] = if is_equal { 1 } else { 0 };
      },
      Opcode::Halt => {
        break
      }
    }

    if should_increment_pointer {
      instruction_pointer += num_params + 1;
    }

    if DEBUG {
      println!("\tUpdated state: {:?}", values);
      println!("\tinstruction_pointer: {:?}", instruction_pointer);
    }

    iteration_count += 1;
    if iteration_count >= MAX_LOOP_ITERATIONS {
      panic!("MAX_LOOP_ITERATIONS exceeded. Aborting.. ");
    }

    if instruction_pointer >= values.len() {
      println!("Unexpected... ");
      break
    }
  }

  output
}

fn get_param_val(values: &Vec<isize>, param: &Parameter) -> isize {
  match param.mode {
    ParameterMode::Position => values[param.value as usize],
    ParameterMode::Immediate => param.value,
  }
}


#[derive(Debug)]
struct Parameter {
  value: isize,
  mode: ParameterMode,
}

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate,
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
  Halt,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_with_op_codes_1_2_99() {
    struct Case {
      input: Vec<isize>,
      result: Vec<isize>,
    }

    // Example programs from Day 2
    let cases = vec![
      Case {
        input:  vec![1,0,0,0,99],
        result: vec![2,0,0,0,99],
      },
      Case {
        input:  vec![2,3,0,3,99],
        result: vec![2,3,0,6,99],
      },
      Case {
        input:  vec![2,4,4,5,99,0],
        result: vec![2,4,4,5,99,9801],
      },
      Case {
        input:  vec![1,1,1,4,99,5,6,0,99],
        result: vec![30,1,1,4,2,5,6,0,99],
      },
      Case {
        input:  vec![1,9,10,3,2,3,11,0,99,30,40,50],
        result: vec![3500,9,10,70,2,3,11,0,99,30,40,50],
      },
    ];

    for case in cases {
      let mut input = case.input.clone();
      run_intcode_program(&mut input, 0);
      assert_eq!(input, case.result);
    }
  }

  struct TestCase {
    program: Vec<isize>,
    input: isize,
    final_state: Vec<isize>,
    output: Vec<isize>,
  }

  fn run_test_cases(cases: &Vec<TestCase>) {
    for case in cases {
      let mut program = case.program.clone();
      let output = run_intcode_program(&mut program, case.input);
      assert_eq!(program, case.final_state);
      assert_eq!(output, case.output);
    }
  }

  #[test]
  fn op_codes_3_and_4() {
    let cases = vec![
      TestCase {
        program: vec![3,0,4,0,99],
        input: 1,
        final_state: vec![1, 0, 4, 0, 99],
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
        input: -1,
        final_state: vec![1005, 8, 5, 4, 9, 4, 10, 99, 1, -1, -2],
        output: vec![-2],
      },
      TestCase {
        // Address 8 is 0, so it does NOT jump.
        program: vec![1005, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
        input: -1,
        final_state: vec![1005, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
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
        input: -1,
        final_state: vec![1006, 8, 5, 4, 9, 4, 10, 99, 1, -1, -2],
        output: vec![-1, -2],
      },
      TestCase {
        // Address 8 is 0, so it does jump.
        program: vec![1006, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
        input: -1,
        final_state: vec![1006, 8, 5, 4, 9, 4, 10, 99, 0, -1, -2],
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
        input: 8, // equals 8
        final_state: vec![3,9,8,9,10,9,4,9,99,1,8],
        output: vec![1],
      },
      TestCase {
        program: program1.clone(),
        input: 5, // less than 8
        final_state: vec![3,9,8,9,10,9,4,9,99,0,8],
        output: vec![0],
      },
      TestCase {
        program: program1.clone(),
        input: 900, // greater than 8
        final_state: vec![3,9,8,9,10,9,4,9,99,0,8],
        output: vec![0],
      },

      // immediate mode (3rd bulleted example in part 2)
      TestCase {
        program: program2.clone(),
        input: 8, // equals 8,
        final_state: vec![3,3,1108,1,8,3,4,3,99],
        output: vec![1],
      },
      TestCase {
        program: program2.clone(),
        input: 5, // less than 8,
        final_state: vec![3,3,1108,0,8,3,4,3,99],
        output: vec![0],
      },
      TestCase {
        program: program2.clone(),
        input: 900, // greater than 8,
        final_state: vec![3,3,1108,0,8,3,4,3,99],
        output: vec![0],
      },
    ];

    run_test_cases(&cases);
  }
}
