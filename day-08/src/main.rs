use std::fs;

const BLACK: u32 = 0;
const WHITE: u32 = 1;
const TRANSARENT: u32 = 2;

fn main() {
  let input = fs::read_to_string("./puzzle-input.txt")
    .expect(&format!("Problem reading file"))
    .trim()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

  let layers = parse_layers(&input, 25, 6);
  solve_part_1(&layers);
  solve_part_2(&layers);
}

fn solve_part_2(layers: &Vec<Vec<u32>>) {
  let mut decoded_img: Vec<u32> = Vec::new();

  let layer_size = layers[0].len();
  for i in 0..layer_size {
    let mut done = false;
    for layer in layers.iter() {
      if layer[i] != TRANSARENT {
        decoded_img.push(layer[i]);
        done = true;
        break
      }
    }
    if !done {
      decoded_img.push(TRANSARENT);
    }
  }

  println!("");
  for row in decoded_img.chunks(25) {
    let pretty_row: String = row
      .iter()
      .map(|x| match *x { WHITE => '#', _ => ' ' })
      .collect();
    println!("{}", pretty_row);
  }
  println!("");
}


fn solve_part_1(layers: &Vec<Vec<u32>>) {
  let mut min_zero_count = u32::max_value();
  let mut layer_with_min_zeros: &Vec<u32> = &layers[0];

  for layer in layers.iter() {
    let num_zeros = count_num(&layer, 0);
    if num_zeros < min_zero_count {
      min_zero_count = num_zeros;
      layer_with_min_zeros = layer;
    }
  }

  println!(
    "Part 1 Answer: {:?}",
    count_num(&layer_with_min_zeros, 1) * count_num(&layer_with_min_zeros, 2),
  );
}

fn count_num(nums: &Vec<u32>, target: u32) -> u32 {
  // This doesn't work, not sure why:
  //   nums.iter().filter(|x| **x= target).collect().len()
  let mut count = 0;
  for num in nums {
    if *num == target {
      count += 1;
    }
  }
  count
}

fn parse_layers(input: &Vec<u32>, width: u32, height: u32) -> Vec<Vec<u32>> {
  let layer_size: usize = (width * height) as usize;
  assert!(input.len() % layer_size == 0);

  let num_layers = input.len() / layer_size;
  let mut layers = Vec::new();
  for z in 0..num_layers {
    let start = z * layer_size;
    layers.push(input[start..start+layer_size].to_vec())
  }
  layers
}
