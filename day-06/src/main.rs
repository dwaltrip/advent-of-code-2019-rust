use std::collections::HashMap;
use std::fs;

fn main() {
  let raw_input = fs::read_to_string("./puzzle-input.txt")
    .expect("Something went wrong reading the file");

  let orbit_declarations: Vec<&str> = raw_input
    .split('\n')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect();

  let orbit_map = build_orbit_map(&orbit_declarations);
  let count = count_orbits(&orbit_map);

  println!("count_orbits: {:?}", count);
}

fn count_orbits(orbit_map: &OrbitMap) -> u32 {
  count_orbits_for(&orbit_map, "COM", 0)
}

fn count_orbits_for(orbit_map: &OrbitMap, target: &str, current_count: u32) -> u32 {
  match orbit_map.children_for_parent.get(target) {
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
  let mut children_for_parent: HashMap<String, Vec<String>> = HashMap::new();

  for orbit in orbit_declarations.iter() {
    let parts: Vec<&str> = orbit.split(")").collect();
    if parts.len() != 2 {
      panic!("Invalid orbit: {:?}", orbit);
    }
    let parent = parts[0];
    let child = parts[1];

    let children = children_for_parent
      .entry(parent.to_string())
      .or_insert(Vec::new());
    children.push(child.to_string());
  }

  OrbitMap { children_for_parent }
}

struct OrbitMap {
  children_for_parent: HashMap<String, Vec<String>>,
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn name() {
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
}
