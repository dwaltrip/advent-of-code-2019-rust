use std::fs;


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


fn solve_part_2(original_program: &Vec<usize>) {
  for val1 in 0..100 {
    for val2 in 0..100 {
      let mut program = original_program.clone();

      program[1] = val1;
      program[2] = val2;
      run_intcode_program(&mut program);

      if program[0] == 19690720 {
        println!("Found the solution!");
        println!("program[1] = {:?}", val1);
        println!("program[2] = {:?}", val2);
        println!("(noun * 100) + verb = {:?}", (val1 * 100) + val2);
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
