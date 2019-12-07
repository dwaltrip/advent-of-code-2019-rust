use std::collections::HashMap;
use std::fs;


struct ProgramInputRange {
  address: usize,
  min: usize,
  max: usize,
}

struct TargetOutput {
  address: usize,
  value: usize,
}


fn main() {
  let input_filename = "./input.txt";

  let raw_input = fs::read_to_string(input_filename)
    .expect("Something went wrong reading the file");

  let program: Vec<usize> = raw_input
    .trim()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();

  // solve_part_1(&program);
  solve_part_2(&program);
}


fn solve_part_1(original_program: &Vec<usize>) {
  let mut program = original_program.clone();

  // In order to restore the program to the "1202 program alarm" state:
  //  * Replace position 1 with "12"
  //  * Replace position 2 with "2"
  program[1] = 12;
  program[2] = 2;

  println!("program, before: {:?}", program);
  run_intcode_program(&mut program);

  println!("-----------------------------");
  println!("program, after: {:?}", program);
}


fn solve_part_2(program: &Vec<usize>) {

  let inputs = vec![
    ProgramInputRange {
      address: 1,
      min: 0,
      max: 99,
    },
    ProgramInputRange {
      address: 2,
      min: 0,
      max: 99,
    },
  ];

  let target = TargetOutput {
    address: 0,
    value: 19690720,
  };

  find_valid_inputs(&program, &target, &inputs);
}


fn find_valid_inputs(
  original_program: &Vec<usize>,
  target: &TargetOutput,
  inputs: &Vec<ProgramInputRange>,
) {

  let mut input_values: HashMap<usize, usize> = inputs
    .clone()
    .iter()
    .map(|input| (input.address, input.min))
    .collect();

  'outer: for current_input in inputs {
    for next_val in current_input.min..(current_input.max+1) {
      // update the current input to the next value
      *input_values.get_mut(&current_input.address).unwrap() = next_val;

      let mut program = original_program.clone();

      let mut debug_str = String::new();

      // set the inputs
      for (address, value) in input_values.iter() {
        program[address.clone()] = value.clone();
        debug_str.push_str(
          &format!("##  program[{}] = {} ##", address, value)
        );
      }
      println!("{}", debug_str);

      run_intcode_program(&mut program);

      if program[target.address] == target.value {
        println!("Found correct input values: {:?}", input_values);
        break 'outer;
      }
    }
  }
}


fn run_intcode_program(program: &mut Vec<usize>) {
  let mut pos = 0;

  while (pos + 4) <= program.len() {
    let opcode = program[pos];
    let input_pos_1 = program[pos+1];
    let input_pos_2 = program[pos+2];
    let output_pos = program[pos+3];

    if opcode == 99 {
      println!("Halting!");
      break;
    }

    let input1 = program[input_pos_1];
    let input2 = program[input_pos_2];

    program[output_pos] = match opcode {
      1 => input1 + input2,
      2 => input1 * input2,
      _ => panic!("Invalid opcode {:?} at pos {}", opcode, pos),
    };

    pos += 4;
  }
}
