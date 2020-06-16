use std::collections::{ HashMap, HashSet };
use std::fs;
use std::hash::{Hash, Hasher};
use std::cmp::{Eq, PartialEq};

type Point = (isize, isize);

struct AsteroidMap {
  asteroids: HashSet<Point>,
}

fn main() {
  let map = parse_asteroid_map(
    &fs::read_to_string("./puzzle-input.txt").unwrap()
  );
  // solve_part_1(&map);
  solve_part_2(&map);
}

#[allow(dead_code)]
fn solve_part_1(map: &AsteroidMap) {
  println!("best position: {:?}", find_best_observing_pos(&map));
}

#[allow(dead_code)]
fn solve_part_2(map: &AsteroidMap) {
  let (station_pos, _) = find_best_observing_pos(&map);
  println!(
    "200th asteroid: {:?}", 
    &find_nth_asteroid_to_blast(&map, station_pos, 200),
  );
}

fn find_nth_asteroid_to_blast(
  map: &AsteroidMap, station_pos: Point, n: usize
) -> Option<Point> {
  let asteroid_coords: Vec<PolarCoord> = map.asteroids
    .iter()
    .filter(|&p| *p != station_pos)
    .map(|p| PolarCoord::from_point(relativize_point(&station_pos, &p)))
    .collect();

  let mut asteroids_by_angle = gather_by_angle_and_sort_by_distance(&asteroid_coords);
  // NOTE: "up" is at 270 degrees, because the y-axis is inverted.
  let angles = sort_angles_clockwise_starting_at_270(
    &asteroids_by_angle.keys().cloned().collect::<Vec<Angle>>()
  );

  let mut count = 0;
  loop {
    let mut done = true;

    for angle in angles.iter() {
      if let Some(coords) = asteroids_by_angle.get_mut(angle) {
        if let Some(coord) = coords.pop() {
          count += 1;
          if count == n {
            return Some(unrelativize_coords(&station_pos, &coord.point));
          }
        }
      }
      if asteroids_by_angle.get(angle).map_or(0, |coords| coords.len()) > 0 {
        done = false;
      }
    }

    if done {
      break;
    }
  }
  None
}

#[derive(Clone)]
#[derive(Debug)]
struct Angle {
  value: f64,
  rise: isize,
  run: isize,
}

#[derive(Clone)]
#[derive(Debug)]
struct PolarCoord {
  angle: Angle,
  distance: f64,
  point: Point,
}

impl Angle {
  fn from_point(point: Point) -> Angle {
    let gcd = gcd(point.0, point.1) as isize;
    Angle {
      value: angle360(point.1 as f64, point.0 as f64),
      rise: point.1 / gcd,
      run: point.0 / gcd,
    }
  }
}

impl Hash for Angle {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.rise.hash(state);
    self.run.hash(state);
  }
}

impl PartialEq for Angle {
  fn eq(&self, other: &Self) -> bool {
    self.rise == other.rise && self.run == other.run
  }
}

impl Eq for Angle {}

impl PolarCoord {
  fn from_point(point: Point) -> PolarCoord {
    let (x, y) = point;
    PolarCoord {
      angle: Angle::from_point(point),
      distance: ((x.pow(2) + y.pow(2)) as f64).sqrt(),
      point,
    }
  }
}

impl PartialEq for PolarCoord {
  fn eq(&self, other: &Self) -> bool {
    self.angle == other.angle && self.distance == other.distance
  }
}

impl Eq for PolarCoord {}

fn relativize_point(relative_origin: &Point, point: &Point) -> Point {
  (
    point.0 - relative_origin.0,
    point.1 - relative_origin.1,
  )
}

fn unrelativize_coords(relative_origin: &Point, point: &Point) -> Point {
  (
    point.0 + relative_origin.0,
    point.1 + relative_origin.1,
  )
}

fn angle360(y: f64, x: f64) -> f64 {
  let mut theta = y.atan2(x) * (180.0 / std::f64::consts::PI);
  // normalize to range: [0, 360)
  if theta < 0.0 {
    theta += 360.0;
  }
  theta
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
  let slope_gcd = gcd(slope_x, slope_y) as isize;

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
      get_nums_in_between(isize_to_usize(p1.0), isize_to_usize(p2.0), x_step)
    };
  let y_vals = 
    if slope_y == 0 {
      // y values are the same, could use either point
      vec![p1.1; x_dist-1]
    } else {
      get_nums_in_between(isize_to_usize(p1.1), isize_to_usize(p2.1), y_step)
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
fn get_nums_in_between(a: usize, b: usize, step: isize) -> Vec<isize> {
  assert!(a != b);
  assert!(if a < b { step > 0 } else { step < 0 });
  let a = a as isize;
  let b = b as isize;

  let mut nums = Vec::new();
  let is_not_done = |curr| if a < b { curr < b } else { curr > b };

  let mut curr = a + step;
  while is_not_done(curr) {
    nums.push(curr);
    curr += step;
  }
  nums
}

fn parse_asteroid_map(input_string: &str) -> AsteroidMap {
  let mut asteroids = HashSet::new();

  input_string
    .trim()
    .split('\n')
    .enumerate()
    .for_each(|(y, line)| {
      line.trim().chars().enumerate().for_each(|(x, c)| {
        if c == '#' {
          asteroids.insert((x as isize, y as isize));
        }
      });
    });

  AsteroidMap { asteroids }
}

fn gather_by_angle_and_sort_by_distance(
  coords: &[PolarCoord]
) -> HashMap<Angle, Vec<PolarCoord>> {
  let mut map = HashMap::new();

  for coord in coords.iter() {
    map
      .entry(coord.angle.clone())
      .or_insert(Vec::new())
      .push(coord.clone());
  }
  for (_, coords_at_angle) in map.iter_mut() {
    coords_at_angle.sort_by(|a, b| {
      a.distance.partial_cmp(&b.distance).unwrap()
    });
    coords_at_angle.reverse();
  } 

  map
}

// ---------------------------------------------------------------------------

fn gcd(a: isize, b: isize) -> isize {
  let a = a.abs();
  let b = b.abs();

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

fn sort_angles_clockwise_starting_at_270(angles: &[Angle]) -> Vec<Angle> {
  // NOTE: The y-axis is inverted for the grids in this puzzle.
  // Angles in degrees normally go from [0 to 360) in counter-clockwise fashion.
  // With an inverted y-axis, they go in clockwise fashion.
  let zeroth_angle = 270.0;

  let adjust_angle = |theta| {
    // For zeroth angle to become the new zero, we need "greater than or equal to",
    //   not just "greater than".
    // I think if the y-axis was not inverted, we would want "greater than".
    if theta >= zeroth_angle {
      theta - zeroth_angle
    }
    else {
      theta + (360.0 - zeroth_angle)
    }
  };

  let mut copy = angles.to_vec();
  copy.sort_by(|a, b| {
    let a_val = adjust_angle(a.value);
    let b_val = adjust_angle(b.value);
    a_val.partial_cmp(&b_val).unwrap()
  });
  copy
}

// ----------------------------------------------------------------------------

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

  #[test]
  fn test_gather_by_angle_and_sort_by_distance() {
    let points = vec![
      (0, 2),
      (-2, 4),
      (-1, 2),
      (-2, 2),

      (-1, 0),
      (-3, 0),
      (-2, -4),
      (-2, -3),

      (1, -1),
      (3, -1),
      (6, -2),

      (2, 1),
      (3, 0),
      (3, 3),
      (2, 2),
    ];
    let points_as_coords: Vec<PolarCoord> = points
      .iter()
      .map(|&p| PolarCoord::from_point(p))
      .collect();

    let mut expected = HashMap::new();
    let mut add_entry = |key_point: Point, points: Vec<Point>| {
      let p_coord = PolarCoord::from_point(key_point.clone());
      expected.insert(
        p_coord.angle.clone(),
        points.iter().map(|&p| PolarCoord::from_point(p)).collect(),
      );
    };

    add_entry((0, 2), vec![(0, 2)]);
    add_entry((-1, 2), vec![(-2, 4), (-1, 2)]);
    add_entry((-2, 2), vec![(-2, 2)]);

    add_entry((-1, 0), vec![(-3, 0), (-1, 0)]);
    add_entry((-2, -3), vec![(-2, -3)]);
    add_entry((-2, -4), vec![(-2, -4)]);

    add_entry((1, -1), vec![(1, -1)]);
    add_entry((3, -1), vec![(6, -2), (3, -1)]);

    add_entry((3, 0), vec![(3, 0)]);
    add_entry((2, 1), vec![(2, 1)]);
    add_entry((2, 2), vec![(3, 3), (2, 2)]);

    let actual = gather_by_angle_and_sort_by_distance(&points_as_coords);

    assert_eq!(actual.len(), expected.len());
    assert_eq!(actual, expected);
  }

  #[test]
  fn test_find_nth_asteroid_to_blast() {
    let input = String::from("
      .#....#####...#..
      ##...##.#####..##
      ##...#...#.#####.
      ..#.....X...###..
      ..#.#.....#....##
    ");
    let map = parse_asteroid_map(&input);
    let station = (8, 3);

    let asteroids_by_blast_order = vec![
      // first 9
      (8, 1), (9, 0), (9, 1), (10, 0), (9, 2), // 1-5
        (11, 1), (12, 1), (11, 2), (15, 1), // 6-9
      // second 9
      (12, 2), (13, 2), (14, 2), (15, 2), (12, 3), // 10-14
        (16, 4), (15, 4), (10, 4), (4, 4), // 15-18
    ];

    for (n, &pos) in asteroids_by_blast_order.iter().enumerate() { 
      assert_eq!(find_nth_asteroid_to_blast(&map, station, n+1).unwrap(), pos);
    }
  }

  #[test]
  fn test_find_nth_asteroid_to_blast_part_2() {
    let input = String::from("
      .#..##.###...#######
      ##.############..##.
      .#.######.########.#
      .###.#######.####.#.
      #####.##.#.##.###.##
      ..#####..#.#########
      ####################
      #.####....###.#.#.##
      ##.#################
      #####.##.###..####..
      ..######..##.#######
      ####.##.####...##..#
      .#####..#.######.###
      ##...#.##########...
      #.##########.#######
      .####.#.###.###.#.##
      ....##.##.###..#####
      .#.#.###########.###
      #.#.#.#####.####.###
      ###.##.####.##.#..##
    ");
    let map = parse_asteroid_map(&input);

    assert_eq!(
      find_nth_asteroid_to_blast(&map, (11, 13), 200),
      Some((8, 2)),
    );
  }

  #[test]
  fn test_angle360() {
    assert_eq!(Angle::from_point((1, 0)).value, 0.0);
    assert_eq!(Angle::from_point((2, 0)).value, 0.0);

    assert_eq!(Angle::from_point((1, 1)).value, 45.0);
    assert_eq!(Angle::from_point((0, 1)).value, 90.0);

    assert_eq!(Angle::from_point((-1, 1)).value, 135.0);
    assert_eq!(Angle::from_point((-1, 0)).value, 180.0);

    assert_eq!(Angle::from_point((0, -1)).value, 270.0);
    assert_eq!(Angle::from_point((1, -1)).value, 315.0);
  }

  #[test]
  fn test_relativize_point() {
    let station = (8, 3);
    assert_eq!(relativize_point(&station, &(8, 0)), (0, -3));
    assert_eq!(relativize_point(&station, &(10, 3)), (2, 0));
    assert_eq!(relativize_point(&station, &(9, 4)), (1, 1));
    assert_eq!(relativize_point(&station, &(8, 4)), (0, 1));
    assert_eq!(relativize_point(&station, &(8, 2)), (0, -1));
    assert_eq!(relativize_point(&station, &(6, 1)), (-2, -2));
  }

  #[test]
  fn test_sort_angles_clockwise_starting_at_270() {
    let points = vec![
      (0, -2),
      (2, -1),
      (3, 0),
      (3, 1),
      (0, 2),
      (-2, 3),
      (-3, 0),
      (-2, -1), 
      (-1, -3), 
    ];
    let angles: Vec<Angle> = points
      .iter()
      .map(|&p| Angle::from_point(p))
      .collect();

    let sorted = sort_angles_clockwise_starting_at_270(&angles);
    assert_eq!(angles, sorted);
  }
}
