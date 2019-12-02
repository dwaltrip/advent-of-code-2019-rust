use std::fs;

fn main() {
  println!("total fuel needed: {}", calculate_total_fuel("./src/input.txt"))
}

fn calculate_total_fuel(input_filename: &str) -> u32 {
  let contents = fs::read_to_string(input_filename)
    .expect("Something went wrong reading the file");

  let lines: Vec<&str> = contents.split('\n').collect();

  let mut fuel_amounts: Vec::<u32> = Vec::new();

  for line in lines.iter() {
    let line_cleaned = line.trim();
    if line_cleaned.chars().count() > 0 {
      let fuel_needed = fuel_for_module(line_cleaned.parse().unwrap());
      fuel_amounts.push(fuel_needed);
    }
  }

  fuel_amounts.iter().sum()
}

fn fuel_for_module(mass: u32) -> u32 {
  (((mass / 3) as f64).floor() as u32) - 2
}
