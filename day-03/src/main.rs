use std::cmp;
use std::fs;
use std::ops::Range;
use std::time::Instant;

fn main() {
  let input_filename = "./input.txt";
  let raw_input = fs::read_to_string(input_filename)
    .expect("Something went wrong reading the file");

  let wire_paths: Vec<&str> = raw_input.trim().split('\n').collect();

  solve_part_1(&wire_paths[0], &wire_paths[1]);
}

fn solve_part_1(path1: &str, path2: &str) {
  let t1 = Instant::now();
  let lines1 = parse_paths(&path1);
  let lines2 = parse_paths(&path2);
  println!("Time to parse paths: {:.2?}", t1.elapsed());

  let mut intersections: Vec<Point> = Vec::new();

  let t2 = Instant::now();
  for line1 in lines1.iter() {
    for line2 in lines2.iter() {
      if let Some(point) = get_intersection(&line1, &line2) {
        intersections.push(point);
      }
    }
  }
  println!("Time to find intersections: {:.2?}", t2.elapsed());

  let mut ordered_intersections: Vec<&Point> = intersections.iter().collect();
  ordered_intersections.sort_by_key(|point| point_size(&point));

  println!("-----------------------------");
  println!("Ordered Intersections:");
  for point in ordered_intersections[1..11].iter() {
    println!("{:?}", point);
  }
}

// Structs

#[derive(Debug)]
#[derive(Clone)]
struct Point { x: i32, y: i32 }

#[derive(Debug)]
struct Line {
  p1: Point,
  p2: Point,
  x_range: Range<i32>,
  y_range: Range<i32>,
}

impl Line {
  fn new(p1: &Point, p2: &Point) -> Line {
    assert!(p1.x == p2.x || p1.y == p2.y, "Expected grid line");
    let x_range = make_range(p1.x, p2.x);
    let y_range = make_range(p1.y, p2.y);
    let p1 = p1.clone();
    let p2 = p2.clone();
    Line { p1, p2, x_range, y_range }
  }
}

// Main logic

fn get_intersection(line1: &Line, line2: &Line) -> Option<Point> {
  let x_overlap_opt = range_overlap(&line1.x_range, &line2.x_range);
  let y_overlap_opt = range_overlap(&line1.y_range, &line2.y_range);

  match (x_overlap_opt, y_overlap_opt) {
    (Some(x_overlap), Some(y_overlap)) => {
      Some(Point { x: x_overlap.start, y: y_overlap.start })
    }
    _ => None,
  }
}

#[derive(Debug)]
enum GridStep {
  Dx(i32),
  Dy(i32),
}

fn parse_paths(raw_path: &str) -> Vec<Line> {
  let steps: Vec<GridStep> = raw_path
    .split(',')
    .map(|path_step| parse_path_step(path_step))
    .collect();

  let mut points = vec![Point { x: 0, y: 0 }];
  for step in steps {
    let curr = &points.last().unwrap();
    let next = match step {
      GridStep::Dx(dx) => Point { x: curr.x + dx, y: curr.y },
      GridStep::Dy(dy) => Point { x: curr.x,      y: curr.y + dy },
    };
    points.push(next);
  }

  let mut lines = Vec::new();
  {
    let mut iter_points = points.iter();
    let mut prev = iter_points.next().unwrap();
    for next in iter_points {
      lines.push(Line::new(&prev, &next));
      prev = next;
    }
  }
  lines
}

fn parse_path_step(step: &str) -> GridStep {
  let direction = &step[0..1];
  let amount = (&step[1..]).parse().unwrap();
  match direction {
    "R" => GridStep::Dx(amount),
    "L" => GridStep::Dx(-amount),
    "U" => GridStep::Dy(amount),
    "D" => GridStep::Dy(-amount),
    _ => panic!("Invalid direction: {:?}", direction),
  }
}

// Helpers

fn range_overlap(r1: &Range<i32>, r2: &Range<i32>) -> Option<Range<i32>> {
  if (r1.end <= r2.start) || (r2.end <= r1.start) {
    None
  } else {
    let start = cmp::max(r1.start, r2.start);
    let end = cmp::min(r1.end, r2.end);
    Some(start..end)
  }
}

fn make_range(a: i32, b: i32) -> Range<i32> {
  if a <= b {
    (a..(b+1))
  } else {
    (b..(a+1))
  }
}

fn point_size(point: &Point) -> i32 {
  point.x.abs() + point.y.abs()
}

fn prettify_line(line: &Line) -> String {
  format!("Line {{ p1: ({}, {}) p2: ({}, {}) }}",
    line.p1.x, line.p1.y, line.p2.x, line.p2.y)
}
