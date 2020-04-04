
pub fn get_digits(num: &u32) -> Vec<u32> {
  num
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).expect(&format!("{:?} is not a digit", c)))
    .collect()
}

// This has OK performance. It increments 1 to 1,000,000 in 170 milliseconds.
pub fn increment_digits(digits: &mut Vec<u32>) {
  let mut all_nines = true;

  for i in (0..digits.len()).rev() {
    if digits[i] < 9 {
      digits[i] += 1;
      all_nines = false;
      break;
    }
    else {
      digits[i] = 0;
    }
  }

  if all_nines {
    digits.insert(0, 1);
  }
}

// This is faster (simple testing showed about 2.5x faster).
// BUT it is also much more complex and brittle compared to `get_digits`.
#[allow(dead_code)]
pub fn get_digits_fast(num: &u32) -> Vec<u32> {
  let mut digits_reversed = Vec::new();  

  let mut power = 1;
  let mut remainder = num % 10;
  digits_reversed.push(remainder);

  while remainder < *num {
    let prev_remainder = remainder;
    power = power + 1;
    remainder = num % 10_u32.pow(power);
    digits_reversed.push(
      (remainder - prev_remainder) / 10_u32.pow(power - 1)
    );
  }

  digits_reversed.reverse();
  digits_reversed
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn both_approaches_work() {
    let numbers = vec![
      572,
      1_123,
      9_844,
      502_341,
    ];

    let expected: Vec<Vec<u32>> = vec![
      vec![5, 7, 2],
      vec![1, 1, 2, 3],
      vec![9, 8, 4, 4],
      vec![5, 0, 2, 3, 4, 1],
    ];

    for (number, digits) in numbers.iter().zip(expected.iter()) {
      assert_eq!(&get_digits(number), digits);
      assert_eq!(&get_digits_fast(number), digits);
    }
  }

  #[test]
  fn increments_properly() {
    struct Case {
      number: Vec<u32>,
      expected: Vec<u32>,
    }

    let cases = vec![
      Case { number: vec![0], expected: vec![1] },
      Case { number: vec![5], expected: vec![6] },
      Case { number: vec![9], expected: vec![1, 0] },
      Case { number: vec![5, 1], expected: vec![5, 2] },
      Case { number: vec![9, 9], expected: vec![1, 0, 0] },
      Case { number: vec![1, 0, 0], expected: vec![1, 0, 1] },
      Case { number: vec![8, 4, 5, 9], expected: vec![8, 4, 6, 0] },
      Case { number: vec![4, 5, 3, 2, 1, 1], expected: vec![4, 5, 3, 2, 1, 2] },
      Case { number: vec![9, 9, 9, 9, 9, 9], expected: vec![1, 0, 0, 0, 0, 0, 0] },
    ];

    for case in cases {
      let mut number = case.number;
      increment_digits(&mut number);
      assert_eq!(number, case.expected);
    }
  }
}
