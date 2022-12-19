#![allow(unused_variables)]

const INPUT: &str = include_str!("input");

pub(crate) fn part_1(input: &str) -> u32 {
  todo!()
}

pub(crate) fn part_2(input: &str) -> u32 {
  todo!()
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
    assert_eq!(res, 0);
  }

  #[test]
  #[ignore = "later"]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 0)
  }
}
