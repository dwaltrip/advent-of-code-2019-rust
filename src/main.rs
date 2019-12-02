use std::fs;

fn main() {
  println!("Total fuel needed: {}", calculate_total_fuel("./src/input.txt"));
}


fn calculate_total_fuel(input_filename: &str) -> i32 {
  let module_masses = fs::read_to_string(input_filename)
    .expect("Something went wrong reading the file");

  let lines: Vec<&str> = module_masses.split('\n').collect();

  // NOTE: I did some rudimentary timing and this seems to be slightly faster
  // than the naive for loop with if statement that pushes into a mut vector,
  // which is awesome. I thought it might be slower.
  lines.iter()
    .map(|s| s.trim())
    .filter(|s| s.chars().count() > 0)
    .map(|mass| calculate_fuel_for_component(
      mass.parse().expect(&format!("Failed to parse: {:?}", mass))
    ))
    .sum()
}


fn calculate_fuel_for_component(component_mass: i32) -> i32 {
  let fuel_mass = (((component_mass as f64) / 3.0).floor() as i32) - 2;

  // Part 2: account for the fuel needed to lift the additional fuel
  if fuel_mass <= 0 {
    0
  } else {
    fuel_mass + calculate_fuel_for_component(fuel_mass)
  }
}
