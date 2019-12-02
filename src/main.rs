use std::fs;

fn main() {
  println!("Total fuel needed: {}", calculate_total_fuel("./src/input.txt"));
}


fn calculate_total_fuel(input_filename: &str) -> u32 {
  let module_masses = fs::read_to_string(input_filename)
    .expect("Something went wrong reading the file");

  let lines: Vec<&str> = module_masses.split('\n').collect();

  // NOTE: I did some rudimentary timing and this seems to be slightly faster
  // than the naive for loop with if statement that pushes into a mut vector,
  // which is awesome. I thought it might be slower.
  lines.iter()
    .map(|s| s.trim())
    .filter(|s| s.chars().count() > 0)
    .map(|mass| fuel_for_module(
      mass.parse().expect(&format!("Failed to parse: {:?}", mass))
    ))
    .sum()
}


fn fuel_for_module(mass: u32) -> u32 {
  (((mass / 3) as f64).floor() as u32) - 2
}
