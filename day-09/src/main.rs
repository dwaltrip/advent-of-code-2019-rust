use std::fs;

mod intcode_computer;

fn main() {
  let input = parse_program_from_file("./puzzle-input.txt");
  solve_part_1(&input);
}

fn solve_part_1(input: &[isize]) {
  let mut program = intcode_computer::Program::new(&input);
  let output = program.run(&[1]);
  println!("output: {:?}", output);
}

fn parse_program_from_file(filename: &str) -> Vec<isize> {
  fs::read_to_string(&filename)
    .expect(&format!("Problem reading file: {:?}", &filename))
    .trim()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect()
}
