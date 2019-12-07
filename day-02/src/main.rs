use std::fs;

fn main() {
  let input_filename = "./input.txt";

  let raw_input = fs::read_to_string(input_filename)
    .expect("Something went wrong reading the file");

  let mut program: Vec<usize> = raw_input
    .trim()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect();

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
