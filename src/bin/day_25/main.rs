#![allow(unused_variables)]

const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Snafu(String);

fn todec(s: &str) -> usize {
  s.chars().fold(0, |n, d| {
    n * 5 + "=-012".chars().position(|x| x == d).unwrap() - 2
  })
}

fn tosnafu(n: usize) -> String {
  if n == 0 {
    String::new()
  } else {
    tosnafu((n + 2) / 5) + ["0", "1", "2", "=", "-"][n % 5]
  }
}

pub(crate) fn part_1(input: &str) -> String {
  tosnafu(input.lines().map(todec).sum())
}

fn main() {
  println!("Part 1: {}", part_1(&INPUT));
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = include_str!("sample");

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, String::from("2=-1=0"));
  }
}
