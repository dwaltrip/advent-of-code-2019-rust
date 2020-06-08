use std::cmp;
use std::collections::{ HashMap, HashSet };
use std::fs;

type Point = (usize, usize);

struct AsteroidMap {
  asteroids: HashSet<Point>,
  height: usize,
  width: usize,
}

fn main() {
  let map = parse_asteroid_map(
    &fs::read_to_string("./puzzle-input.txt").unwrap()
  );

  solve_part_1(&map);
}

fn solve_part_1(map: &AsteroidMap) {
  println!("best position: {:?}", find_best_observing_pos(&map));
}

fn find_best_observing_pos(map: &AsteroidMap) -> (Point, usize) {
  let mut max = 0;
  let mut best_pos = None;

  for (point, count) in get_visibility_counts(&map) {
    if count > max {
      best_pos = Some(point);
      max = count;
    }
  }
  (best_pos.unwrap(), max)
}

fn get_visibility_counts(map: &AsteroidMap) -> HashMap<Point, usize> {
  let mut counts: HashMap<Point, usize> = HashMap::new();

  for &candidate in map.asteroids.iter() {
    for &asteroid in map.asteroids.iter() {
      if candidate == asteroid {
        continue
      }

      if can_see_each_other(&map, candidate, asteroid) {
        *counts.entry(candidate).or_insert(0) += 1;
      }
    }
  }
  counts
}

fn can_see_each_other(map: &AsteroidMap, p1: Point, p2: Point) -> bool {
  assert!(p1 != p2);

  for blocking_pos in get_points_on_line(p1, p2).iter() {
    if map.asteroids.contains(&blocking_pos) {
      return false
    }
  }
  true
}

// This returns points on the line, EXCLUDING both p1 and p2
fn get_points_on_line(p1: Point, p2: Point) -> Vec<Point> {
  assert!(p1 != p2);

  let slope_x: isize = p2.0 as isize - p1.0 as isize;
  let slope_y: isize = p2.1 as isize - p1.1 as isize;
  let slope_gcd = gcd(slope_x.abs() as usize, slope_y.abs() as usize) as isize;

  if slope_gcd == 1 {
    return vec![];
  }

  let x_dist = slope_x.abs() as usize;
  let y_dist = slope_y.abs() as usize;

  let x_step  = slope_x / slope_gcd;
  let y_step = slope_y / slope_gcd;

  let x_vals = 
    if slope_x == 0 {
      // the x values are the same, could use either point
      vec![p1.0; y_dist-1]
    } else {
      get_nums_in_between(p1.0, p2.0, x_step)
    };
  let y_vals = 
    if slope_y == 0 {
      // y values are the same, could use either point
      vec![p1.1; x_dist-1]
    } else {
      get_nums_in_between(p1.1, p2.1, y_step)
    };
  assert!(x_vals.len() == y_vals.len());

  let mut points: Vec<Point> = x_vals
    .iter()
    .cloned()
    .zip(y_vals.iter().cloned())
    .collect();
  points.sort();
  points
}

// TODO: I'm sure there is a nicer way of doing this...
// Maybe using Range somehow. But this works well, is easy
// to understand, and the types are simple.
fn get_nums_in_between(a: usize, b: usize, step: isize) -> Vec<usize> {
  assert!(a != b);
  assert!(if a < b { step > 0 } else { step < 0 });
  let a = a as isize;
  let b = b as isize;

  let mut nums = Vec::new();
  let is_not_done = |curr| if a < b { curr < b } else { curr > b };

  let mut curr = a + step;
  while is_not_done(curr) {
    nums.push(isize_to_usize(curr));
    curr += step;
  }
  nums
}

fn parse_asteroid_map(input_string: &str) -> AsteroidMap {
  let mut asteroids = HashSet::new();
  let mut height: usize = 0;
  let mut width: usize = 0;

  input_string
    .trim()
    .split('\n')
    .enumerate()
    .for_each(|(y, line)| {
      height = cmp::max(y, height);
      line.trim().chars().enumerate().for_each(|(x, c)| {
        if c == '#' {
          asteroids.insert((x,y));
        }
        width = cmp::max(x, width);
      });
    });

  AsteroidMap { asteroids, height, width }
}

fn gcd(a: usize, b: usize) -> usize {
  if a == 0 {
    b
  }
  else if b == 0 {
    a
  }
  else {
    if a > b { gcd(a - b, b) } else { gcd(a, b - a) }
  }
}

// TODO: What's the idiomatic of doing what I want here?
fn isize_to_usize(num: isize) -> usize {
  if num < 0 {
    panic!(format!("Failed to convert: {:?}", num));
  }
  num as usize
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_parse_asteroid_map() {
    let input_string = "
      .#..#
      .....
      #####
      ....#
      ...##
    ";

    let mut expected = HashSet::new();
    expected.insert((1,0));
    expected.insert((4,0));

    expected.insert((0,2));
    expected.insert((1,2));
    expected.insert((2,2));
    expected.insert((3,2));
    expected.insert((4,2));

    expected.insert((4,3));
    expected.insert((3,4));
    expected.insert((4,4));

    let map = parse_asteroid_map(&input_string);
    assert_eq!(map.asteroids, expected);
    assert_eq!(map.height, 4);
    assert_eq!(map.width, 4);
  }

  #[test]
  fn test_get_visibility_counts() {
    let input_string = "
      .#..#
      .....
      #####
      ....#
      ...##
    ";

    let mut expected_counts = HashMap::new();
    expected_counts.insert((1,0), 7);
    expected_counts.insert((4,0), 7);

    expected_counts.insert((0,2), 6);
    expected_counts.insert((1,2), 7);
    expected_counts.insert((2,2), 7);
    expected_counts.insert((3,2), 7);
    expected_counts.insert((4,2), 5);

    expected_counts.insert((4,3), 7);
    expected_counts.insert((3,4), 8);
    expected_counts.insert((4,4), 7);

    let map = parse_asteroid_map(&input_string);
    assert_eq!(get_visibility_counts(&map), expected_counts);
  }

  #[test]
  fn gcd_works() {
    assert_eq!(gcd(54, 24), 6);
    assert_eq!(gcd(24, 54), 6);
  }

  #[test]
  fn test_get_nums_in_between() {
    assert_eq!(get_nums_in_between(1, 4, 1), vec![2, 3]);
    assert_eq!(get_nums_in_between(4, 1, -1), vec![3, 2]);
    assert_eq!(get_nums_in_between(4, 10, 2), vec![6, 8]);
    assert_eq!(get_nums_in_between(10, 4, -2), vec![8, 6]);
  }

  #[test]
  fn test_get_points_on_line() {
    assert_eq!(get_points_on_line((1,1), (1,2)), vec![]);
    assert_eq!(get_points_on_line((1,1), (2,2)), vec![]);
    assert_eq!(get_points_on_line((1,1), (4,4)), vec![(2,2), (3,3)]);
    assert_eq!(get_points_on_line((4,4), (1,1)), vec![(2,2), (3,3)]);
    assert_eq!(get_points_on_line((2,0), (2,3)), vec![(2,1), (2,2)]);
    assert_eq!(get_points_on_line((2,0), (0,4)), vec![(1,2)]);
  }

  #[test]
  fn test_find_best_observing_pos() {
    struct TestCase {
      input: String,
      best_pos: Point,
      number_visible: usize,
    }

    let cases = vec![
      TestCase {
        input: String::from("
          .#..#
          .....
          #####
          ....#
          ...##
        "),
        best_pos: (3, 4),
        number_visible: 8,
      },
      TestCase {
        input: String::from("
          ......#.#.
          #..#.#....
          ..#######.
          .#.#.###..
          .#..#.....
          ..#....#.#
          #..#....#.
          .##.#..###
          ##...#..#.
          .#....####
        "),
        best_pos: (5, 8),
        number_visible: 33,
      },
      TestCase {
        input: String::from("
          #.#...#.#.
          .###....#.
          .#....#...
          ##.#.#.#.#
          ....#.#.#.
          .##..###.#
          ..#...##..
          ..##....##
          ......#...
          .####.###.
        "),
        best_pos: (1, 2),
        number_visible: 35,
      },
    ];

    for case in cases {
      let map = parse_asteroid_map(&case.input);
      assert_eq!(
        find_best_observing_pos(&map),
        (case.best_pos, case.number_visible),
      );
    }
  }
}
