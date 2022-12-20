#![allow(unused_variables)]

use std::collections::VecDeque;

const INPUT: &str = include_str!("input");
const DECRYPTION_KEY: &str = include_str!("decryption_key");

#[derive(Debug)]
struct Encryption<'a> {
  data: &'a Vec<i64>,
  positions: VecDeque<usize>,
}

impl<'a> Encryption<'a> {
  pub fn new(data: &'a Vec<i64>) -> Self {
    let positions = (0..data.len()).collect::<VecDeque<usize>>();
    Self { data, positions }
  }

  fn mix(&mut self) {
    let modulus = self.positions.len() - 1;
    for (idx, val) in self.data.iter().enumerate() {
      if *val == 0 {
        continue;
      }
      let current_position = self.positions.iter().position(|&x| x == idx).unwrap();
      let old_position = self.positions.remove(current_position).unwrap();
      let new_val = val.unsigned_abs() as usize % modulus;

      if *val < 0 {
        self.positions.rotate_right(new_val);
      } else {
        self.positions.rotate_left(new_val);
      }
      self.positions.insert(current_position, old_position);
    }
  }

  fn coordinates(&self) -> Vec<i64> {
    let idx_0_old = self.data.iter().position(|&x| x == 0).unwrap();
    let idx_0_new = self.positions.iter().position(|&x| x == idx_0_old).unwrap();

    (1..=3)
      .map(|i| {
        let current_position = (i * 1000 + idx_0_new) % self.positions.len();
        self.data[self.positions[current_position]]
      })
      .collect()
  }
}

fn parse(input: &str) -> Vec<i64> {
  input.trim().lines().map(|l| l.parse().unwrap()).collect()
}

pub(crate) fn part_1(input: &str) -> i64 {
  let data = parse(input);
  let mut encryption = Encryption::new(&data);

  encryption.mix();
  encryption.coordinates().iter().sum()
}

pub(crate) fn part_2(input: &str) -> i64 {
  let decryption_key = DECRYPTION_KEY.parse::<i64>().unwrap();
  let data = parse(input).iter().map(|&x| x * decryption_key).collect();
  let mut encryption = Encryption::new(&data);

  (0..10).for_each(|_| encryption.mix());

  encryption.coordinates().iter().sum()
}

fn main() {
  println!("Part 1: {}", part_1(&INPUT));
  println!("Part 2: {}", part_2(&INPUT));
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = include_str!("sample");

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 3);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 1623178306)
  }
}
