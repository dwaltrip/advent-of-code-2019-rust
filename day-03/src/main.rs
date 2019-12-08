use std::cmp;
use std::fs;
use std::ops::Range;


fn main2() {
// fn main() {
  let line1 = Line {
    // p1: Point { x: 3, y: 15 },
    // p2: Point { x: 3, y: 20 },
    p1: Point { x: 7, y: 10 },
    p2: Point { x: 7, y: 20 },
  };
  let line2 = Line {
    // p1: Point { x: 1, y: 15 },
    // p2: Point { x: 3, y: 15 },
    p1: Point { x: 7, y: 20 },
    p2: Point { x: 7, y: 25 },
  };

  println!("line1: {:?}", prettify_line(&line1));
  println!("line1.x_range(): {:?}", pp_range(&line1.x_range()));
  println!("line1.y_range(): {:?}", pp_range(&line1.y_range()));

  println!("line2: {:?}", prettify_line(&line2));
  println!("line2.x_range(): {:?}", pp_range(&line2.x_range()));
  println!("line2.y_range(): {:?}", pp_range(&line2.y_range()));

  println!("get_intersection(line1, line2): {:?}", 
    get_intersection(&line1, &line2)
  );
}

fn pp_range(range: &Range<i32>) -> String {
  format!("({}..{})", range.start, range.end)
}

fn main() {
// fn main2() {
  let input_filename = "./input.txt";

  let raw_input = fs::read_to_string(input_filename)
    .expect("Something went wrong reading the file");

  let wire_paths: Vec<&str> = raw_input.trim().split('\n').collect();

  let lines1 = parse_paths(wire_paths[0]);
  // println!("---------------");
  let lines2 = parse_paths(wire_paths[1]);

  // println!("lines1.len(): {}", lines1.len());
  // println!("lines2.len(): {}", lines2.len());

  let mut intersections: Vec<Point> = Vec::new();

  // let l1 = &lines1[0];
  // let l2 = &lines2[0];

  // println!("-------------------------------------------------");
  // println!("l1: {:?}", prettify_line(&l1));
  // println!("l1.x_range(): {:?}", pp_range(&l1.x_range()));
  // println!("l1.y_range(): {:?}", pp_range(&l1.y_range()));
  // println!("-------------------------------------------------");
  // println!("l2: {:?}", prettify_line(&l2));
  // println!("l2.x_range(): {:?}", pp_range(&l2.x_range()));
  // println!("l2.y_range(): {:?}", pp_range(&l2.y_range()));
  // println!("-------------------------------------------------");
  // println!("get_intersection(&l1, &l2): {:?}", get_intersection(&l1, &l2));

  for line1 in lines1.iter() {
    for line2 in lines2.iter() {
      if let Some(point) = get_intersection(&line1, &line2) {
        intersections.push(point);
      }
    }
  }
  
  println!("intersections.len(): {}", intersections.len());

  let mut ordered_intersections: Vec<&Point> = intersections.iter().collect();
  ordered_intersections.sort_by_key(|point| point_size(&point));

  println!("-----------------------------");
  println!("Ordered Intersections:");
  for point in ordered_intersections[1..11].iter() {
    println!("{:?}", point);
  }
}


fn get_intersection(line1: &Line, line2: &Line) -> Option<Point> {
  let x_overlap_opt = range_overlap(line1.x_range(), line2.x_range());
  let y_overlap_opt = range_overlap(line1.y_range(), line2.y_range());

  // match &x_overlap_opt {
  //   Some(r) => {
  //     println!("x_overlap_opt: {}", pp_range(&r));
  //   },
  //   None => { println!("x_overlap_opt does not match :("); }
  // }
  // match &y_overlap_opt {
  //   Some(r) => {
  //     println!("y_overlap_opt: {}", pp_range(&r));
  //   },
  //   None => { println!("y_overlap_opt does not match :("); }
  // }

  match (x_overlap_opt, y_overlap_opt) {
    (Some(x_overlap), Some(y_overlap)) => {
      Some(Point { x: x_overlap.start, y: y_overlap.start })
    }
    _ => None,
  }
}


fn range_overlap(r1: Range<i32>, r2: Range<i32>) -> Option<Range<i32>> {
  if (r1.end <= r2.start) || (r2.end <= r1.start) {
    None
  } else {
    let start = cmp::max(r1.start, r2.start);
    let end = cmp::min(r1.end, r2.end);
    Some(start..end)
  }
}


#[derive(Debug)]
struct Line {
  p1: Point,
  p2: Point,
}

impl Line {
  fn new(p1: &Point, p2: &Point) -> Line {
    let p1 = p1.clone();
    let p2 = p2.clone();

    if point_size(&p1) <= point_size(&p2) {
      Line { p1, p2 }
    }
    else {
      Line { p1: p2, p2: p1 }
    }
  }

  fn x_range(&self) -> Range<i32> {
    if self.p1.x <= self.p2.x {
      (self.p1.x..(self.p2.x+1))
    } else {
      (self.p2.x..(self.p1.x+1))
    }
  }

  fn y_range(&self) -> Range<i32> {
    if self.p1.y <= self.p2.y {
      (self.p1.y..(self.p2.y+1))
    } else {
      (self.p2.y..(self.p1.y+1))
    }
  }
}


#[derive(Debug)]
#[derive(Clone)]
struct Point { x: i32, y: i32 }

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

  // for line in lines[..10].iter() {
  //   println!("    {}", prettify_line(line));
  // }

  lines

  // println!("lines.first(): {:?} -- lines.last(): {:?}",
  //   lines.first().unwrap(), lines.last().unwrap());

  // NOTE: I don't fully understand this...
  // let mut ordered_lines: Vec<&Line> = lines.iter().collect();

  // ordered_lines.sort_by_key(|line| point_size(&line.p1));

  // ordered_lines

  // println!("ordered_lines.first(): {:?}", ordered_lines.first().unwrap());
  // println!("ordered_lines.last(): {:?}", ordered_lines.last().unwrap());

  // println!("-----------------------------");
  // println!("Sorted Lines:");
  // for line in ordered_lines[..10].iter() {
  //   println!("    {}", prettify_line(line));
  // }

  // let max_point = points.iter().fold(
  //   Point { x: 0, y: 0},
  //   |max, val| if point_size(&val) > point_size(&max) { val.clone() } else { max }
  // );
  // println!("max_point: {:?}", max_point);
}


fn prettify_line(line: &Line) -> String {
  format!("Line {{ p1: ({}, {}) p2: ({}, {}) }}",
    line.p1.x, line.p1.y, line.p2.x, line.p2.y)
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


fn point_size(point: &Point) -> i32 {
  return point.x.abs() + point.y.abs();
}
