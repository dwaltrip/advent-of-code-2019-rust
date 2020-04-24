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

fn solve_part_2(layers: &Vec<&[u32]>) {
  let mut decoded_img: Vec<u32> = Vec::new();

  let layer_size = layers[0].len();
  for i in 0..layer_size {
    // Question: Is this slower than a manual for loop?
    decoded_img.push(layers
      .iter()
      .map(|layer| layer[i])
      .find(|pixel| *pixel != TRANSARENT)
      .unwrap_or(TRANSARENT)
    );
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


fn solve_part_1(layers: &Vec<&[u32]>) {
  let layer_with_min_zeros = layers
    .iter()
    .min_by_key(|layer| count_num(&layer, 0))
    .unwrap();

  println!(
    "Part 1 Answer: {:?}",
    count_num(&layer_with_min_zeros, 1) * count_num(&layer_with_min_zeros, 2),
  );
}

fn count_num(nums: &[u32], target: u32) -> u32 {
  nums.iter().filter(|&x| *x == target).count() as u32
}

fn parse_layers(input: &Vec<u32>, width: u32, height: u32) -> Vec<&[u32]> {
  let layer_size: usize = (width * height) as usize;
  assert!(input.len() % layer_size == 0);
  input.chunks(layer_size).collect()
}
