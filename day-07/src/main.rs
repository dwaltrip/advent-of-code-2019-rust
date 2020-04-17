use itertools::Itertools;
use std::fs;

mod intcode_computer;

fn main() {
  let program = parse_program_from_file("./puzzle-input.txt");
  solve_part_1(&program);
}

fn solve_part_1(program: &Vec<isize>) {
  let phase_settings_values: Vec<isize> = vec![0,1,2,3,4];
  let mut max = 0;

  for phase_settings in phase_settings_values
    .iter()
    .permutations(phase_settings_values.len())
  {
    let output = compute_amplifiers(&program, &phase_settings);
    if output > max {
      max = output;
    }
  }

  println!("max output: {:?}", max);
}

fn compute_amplifiers(
  program: &Vec<isize>,
  phase_settings: &Vec<&isize>,
) -> isize {
  // Initial input of 0, as per puzzle description
  let mut current_input = 0;
  let mut output = vec![];

  for phase_setting_input in phase_settings {
    let mut program = program.clone();
    output = intcode_computer::run_program(
      &mut program,
      // Is it weird that I'm double derefing here? Need to read more about
      // rust iterators, for loops, and idiomatic usage in various contexts.
      &vec![**phase_setting_input, current_input],
    );
    current_input = output[0];
  }

  *output.get(0).expect("Output should have a single value")
}

fn parse_program_from_file(filename: &str) -> Vec<isize> {
  fs::read_to_string(&filename)
    .expect(&format!("Problem reading file: {:?}", &filename))
    .trim()
    .split(',')
    .map(|s| s.parse().unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compute_amplifiers() {
    struct TestCase {
      program: Vec<isize>,
      phase_settings: Vec<isize>,
      output: isize,
    }

    let cases = vec![
      TestCase {
        program: vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
        phase_settings: vec![4,3,2,1,0],
        output: 43210,
      },
      TestCase {
        program: vec![
          3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0,
        ],
        phase_settings: vec![0,1,2,3,4],
        output: 54321,
      },
      TestCase {
        program: vec![
          3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
          1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
        ],
        phase_settings: vec![1,0,4,3,2],
        output: 65210,
      },
    ];

    for case in cases {
      // This useless iter().collect() somehow solves the issue I was having
      //   with Vec<&isize> vs. Vec<isize>
      // In solve_part_1, `permutations()` returns references, but the TestCase struct 
      //   here has owned values.
      // Need to learn why this works... and also the idiomatic way to fix my issue.
      let phase_settings = case.phase_settings.iter().collect();
      let output = compute_amplifiers(&case.program, &phase_settings);
      assert_eq!(output, case.output);
    }
  }
}
