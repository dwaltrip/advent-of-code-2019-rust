use std::fs;

fn main() {
  let raw_input = fs::read_to_string("./puzzle-input.txt")
    .expect("Something went wrong reading the file");

  let mut values: Vec<isize> = raw_input
    .trim()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();

  let output = run_intcode_program(&mut values, 1);
  println!("output: {:?}", output);
}

fn run_intcode_program(values: &mut Vec<isize>, input: isize) -> Vec<isize> {
  let mut output = Vec::new();

  let mut instruction_pointer = 0;
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
      99 => Opcode::Halt,
      _ => panic!("Invalid opcode: {:?}", opcode_num),
    };
    let num_params = match opcode {
      Opcode::Add => 3,
      Opcode::Multiply => 3,
      Opcode::Input => 1,
      Opcode::Output => 1,
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
      Opcode::Halt => {
        break
      }
    }

    instruction_pointer += num_params + 1;

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

  #[test]
  fn op_codes_3_and_4() {
    struct Case {
      program: Vec<isize>,
      input: isize,
      final_state: Vec<isize>,
      output: Vec<isize>,
    }

    let cases = vec![
      Case {
        program: vec![3,0,4,0,99],
        input: 1,
        final_state: vec![1, 0, 4, 0, 99],
        output: vec![1],
      },
    ];

    for case in cases {
      let mut program = case.program.clone();
      let output = run_intcode_program(&mut program, case.input);
      assert_eq!(program, case.final_state);
      assert_eq!(output, case.output);
    }
  }
}
