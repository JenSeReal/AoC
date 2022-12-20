#![allow(unused_variables)]

use std::collections::VecDeque;

const INPUT: &str = include_str!("input");
const DECRYPTION_KEY: &str = include_str!("decryption_key");

#[derive(Debug)]
struct Decryption {
  data: Vec<i64>,
  positions: VecDeque<usize>,
}

impl Decryption {
  pub fn new(data: &Vec<i64>, decryption_key: &i64) -> Self {
    let data = data.iter().map(|&x| x * decryption_key).collect::<Vec<_>>();
    let positions = (0..data.len()).collect::<VecDeque<usize>>();
    Self { data, positions }
  }

  fn mix(&mut self, iterations: usize) {
    let modulus = self.positions.len() - 1;
    (0..iterations).for_each(|_| {
      for (i, &x) in self.data.iter().enumerate() {
        let current_position = self.positions.iter().position(|&y| y == i).unwrap();
        self.positions.remove(current_position);
        let new_index =
          (current_position as i64 + x).rem_euclid(self.positions.len() as i64) as usize;
        self.positions.insert(new_index, i)
      }
    });
  }

  fn coordinates(&self) -> Vec<i64> {
    let idx_0_old = self.data.iter().position(|&x| x == 0).unwrap();
    let idx_0_new = self.positions.iter().position(|&x| x == idx_0_old).unwrap();

    [1000, 2000, 3000]
      .iter()
      .map(|i| self.data[self.positions[(idx_0_new + i) % self.positions.len()]])
      .collect()
  }
}

fn parse(input: &str) -> Vec<i64> {
  input.trim().lines().map(|l| l.parse().unwrap()).collect()
}

pub(crate) fn part_1(input: &str) -> i64 {
  let data = parse(input);
  let mut encryption = Decryption::new(&data, &1);

  encryption.mix(1);
  encryption.coordinates().iter().sum()
}

pub(crate) fn part_2(input: &str) -> i64 {
  let decryption_key = DECRYPTION_KEY.parse::<i64>().unwrap();
  let data = parse(input);
  let mut encryption = Decryption::new(&data, &decryption_key);

  encryption.mix(10);
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
