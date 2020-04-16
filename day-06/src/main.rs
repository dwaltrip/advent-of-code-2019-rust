use std::collections::HashMap;
use std::fs;
use std::fmt::{Debug};

fn main() {
  let raw_input = fs::read_to_string("./puzzle-input.txt")
    .expect("Something went wrong reading the file");

  let orbit_declarations: Vec<&str> = raw_input
    .split('\n')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect();

  let orbit_map = build_orbit_map(&orbit_declarations);

  solve_part_1(&orbit_map);
  solve_part_2(&orbit_map);
}

fn solve_part_1(orbit_map: &OrbitMap) {
  let count = count_orbits(&orbit_map);
  println!("count_orbits: {:?}", count);
}

fn solve_part_2(orbit_map: &OrbitMap) {
  let you_parent = orbit_map.parent_lookup.get("YOU").unwrap();
  let santa_parent = orbit_map.parent_lookup.get("SAN").unwrap();

  let count = count_transfers(&you_parent, &santa_parent, &orbit_map);
  println!("number of transfers: {:?}", count); 
}


fn count_transfers(a: &str, b: &str, orbit_map: &OrbitMap) -> u32 {
  if a == b {
    0
  }
  else {
    let depth_a = orbit_map.depth_lookup.get(a).unwrap();
    let depth_b = orbit_map.depth_lookup.get(b).unwrap();

    let next_a =
      if depth_a >= depth_b {
        orbit_map.parent_lookup.get(a).unwrap()
      }
      else {
        a 
      };
    let next_b =
      if depth_b >= depth_a {
        orbit_map.parent_lookup.get(b).unwrap()
      }
      else {
        b
      };

    count_transfers(&next_a, &next_b, &orbit_map) +
      if a == next_a { 0 } else { 1 } +
      if b == next_b { 0 } else { 1 }
  }
}


fn count_orbits(orbit_map: &OrbitMap) -> u32 {
  count_orbits_for(&orbit_map, "COM", 0)
}

fn count_orbits_for(orbit_map: &OrbitMap, target: &str, current_count: u32) -> u32 {
  match orbit_map.children_lookup.get(target) {
    Some(children) => {
      let mut counts_for_children = 0;
      for child in children.iter() {
        counts_for_children += count_orbits_for(&orbit_map, &child, current_count + 1);
      }
      current_count + counts_for_children
    }
    None => current_count,
  }
}


fn build_orbit_map(orbit_declarations: &Vec<&str>) -> OrbitMap {
  let mut children_lookup: HashMap<String, Vec<String>> = HashMap::new();
  let mut parent_lookup: HashMap<String, String> = HashMap::new();

  for orbit in orbit_declarations.iter() {
    let parts: Vec<&str> = orbit.split(")").collect();
    if parts.len() != 2 {
      panic!("Invalid orbit: {:?}", orbit);
    }
    let parent = parts[0].to_string();
    let child = parts[1].to_string();

    let children = children_lookup
      .entry(parent.clone())
      .or_insert(Vec::new());
    children.push(child.clone());

    parent_lookup.insert(child.clone(), parent.clone());
  }

  let mut depth_lookup: HashMap<String, u32> = HashMap::new();
  calculate_depths("COM", 0, &children_lookup, &mut depth_lookup);

  OrbitMap {
    children_lookup,
    parent_lookup,
    depth_lookup,
  }
}

fn calculate_depths(
  object: &str,
  depth: u32,
  children_lookup: &HashMap<String, Vec<String>>,
  mut depth_lookup: &mut HashMap<String, u32>,
) {
  if let Some(children) = children_lookup.get(object) {
    for child in children.iter() {
      depth_lookup.insert(child.to_string(), depth + 1);
      calculate_depths(child, depth + 1, &children_lookup, &mut depth_lookup);
    }
  }
}


struct OrbitMap {
  children_lookup: HashMap<String, Vec<String>>,
  parent_lookup: HashMap<String, String>,
  depth_lookup: HashMap<String, u32>,
}

impl OrbitMap {
  fn pretty_print(&self) {
    println!("-- OrbitMap --");

    fn print_map<T: Debug, U: Debug>(map: &HashMap<T, U>) {
      for (key, value) in map.iter() {
        println!("\t\t{:?} -> {:?}", key, value);
      }
    }
    println!("\t children_lookup:");
    print_map(&self.children_lookup);
    println!("\t parent_lookup:");
    print_map(&self.parent_lookup);
    println!("\t depth_lookup:");
    print_map(&self.depth_lookup);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_count_orbits() {
    let orbit_declarations = vec![
     "COM)B",
     "B)C",
     "C)D",
     "D)E",
     "E)F",
     "B)G",
     "G)H",
     "D)I",
     "E)J",
     "J)K",
     "K)L",
    ];
    let orbit_map = build_orbit_map(&orbit_declarations);
    assert_eq!(count_orbits(&orbit_map), 42);
  }

  #[test]
  fn test_count_transfers() {
    let orbit_declarations = vec![
      "COM)A",
      "A)B",
      "A)C",
      "C)D",
      "D)E",
      "D)F",
    ];
    let orbit_map = build_orbit_map(&orbit_declarations);
    assert_eq!(count_transfers("B", "F", &orbit_map), 4);
  }
}
