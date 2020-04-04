use std::time::Instant;

use crate::digits;

#[allow(dead_code)]
pub fn get_digits_perf() {
  let t1 = Instant::now();
  for number in 1..1_000_000 {
    let _ = digits::get_digits(&number);
  }
  println!("digits::get_digits -- time: {:.2?}", t1.elapsed());

  let t2 = Instant::now();
  for number in 1..1_000_000 {
    let _ = digits::get_digits_fast(&number);
  }
  println!("digits::get_digits_fast -- time: {:.2?}", t2.elapsed());
}

#[allow(dead_code)]
pub fn increment_digits_perf() {
  let mut digits = vec![0];

  let limit = 1_000_001;

  let t1 = Instant::now();
  for _ in 1..limit+1 {
    digits::increment_digits(&mut digits);
  }
  println!("Time to increment_digits {:?} times: {:?}", limit, t1.elapsed());
}
