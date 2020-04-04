mod digits;
mod perf_test;

fn main() {
  solve_part_1(231832, 767346);

  // run_perf_tests();
}

fn solve_part_1(start: u32, end: u32) {
  let mut digits = digits::get_digits(&start);
  let end_digits = digits::get_digits(&end);

  let mut count = 0;
  loop {
    digits::increment_digits(&mut digits);
    if are_digits_equal(&digits, &end_digits) {
      break;
    }

    if !are_monotonically_increasing(&digits) {
      continue;
    }
    if count_adjacent_pairs(&digits) == 0 {
      continue;
    }

    count += 1;
  }
  println!("Potential passwords count: {:?}", count);
}

fn are_monotonically_increasing(digits: &[u32]) -> bool {
  for i in 0..digits.len()-1 {
    if digits[i] > digits[i+1] {
      return false;
    }
  }
  true
}

fn count_adjacent_pairs(digits: &[u32]) -> u32 {
  let mut count = 0;
  for i in 0..digits.len()-1 {
    if digits[i] == digits[i+1] {
      count += 1;
    }
  }
  count
}

fn are_digits_equal(va: &[u32], vb: &[u32]) -> bool {
  (va.len() == vb.len()) &&  // zip stops at the shortest
   va.iter()
     .zip(vb)
     .all(|(a,b)| *a == *b)
}

#[allow(dead_code)]
fn run_perf_tests() {
  println!("\nPerf test: get_digits (both versions)");
  perf_test::get_digits_perf();

  println!("\nPerf test: increment_digits");
  perf_test::increment_digits_perf();
}
