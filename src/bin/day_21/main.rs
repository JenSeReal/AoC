#![allow(unused_variables)]

use std::collections::{BTreeMap, HashMap};

const INPUT: &str = include_str!("input");

#[derive(Debug)]
enum Yell<'a> {
  Number(u32),
  Add(&'a str, &'a str),
  Sub(&'a str, &'a str),
  Mul(&'a str, &'a str),
  Div(&'a str, &'a str),
}

fn parse(input: &str) -> BTreeMap<&str, Yell> {
  input
    .trim()
    .lines()
    .enumerate()
    .fold(BTreeMap::new(), |mut actions, (i, l)| {
      let (id, yell) = l.split_once(": ").unwrap();
      let split = yell.trim().split_whitespace().collect::<Vec<_>>();
      let yell = match split[..] {
        [lhs, op, rhs] => match op.trim() {
          "+" => Yell::Add(lhs, rhs),
          "-" => Yell::Sub(lhs, rhs),
          "*" => Yell::Mul(lhs, rhs),
          "/" => Yell::Div(lhs, rhs),
          _ => unreachable!(),
        },
        [num] => Yell::Number(num.parse().unwrap()),
        _ => unreachable!(),
      };

      actions.insert(id, yell);
      actions
    })
}

pub(crate) fn part_1(input: &str) -> i64 {
  let yells = parse(input);

  fn calculate_yell<'a>(
    monkey: &'a str,
    monkeys: &'a BTreeMap<&'a str, Yell>,
    cache: &mut HashMap<&'a str, i64>,
  ) -> i64 {
    if let Some(val) = cache.get(monkey) {
      return *val;
    };

    let val = match monkeys.get(monkey).unwrap() {
      Yell::Number(val) => (*val).into(),
      Yell::Add(lhs, rhs) => {
        calculate_yell(lhs, monkeys, cache) + calculate_yell(rhs, monkeys, cache)
      }
      Yell::Sub(lhs, rhs) => {
        calculate_yell(lhs, monkeys, cache) - calculate_yell(rhs, monkeys, cache)
      }
      Yell::Mul(lhs, rhs) => {
        calculate_yell(lhs, monkeys, cache) * calculate_yell(rhs, monkeys, cache)
      }
      Yell::Div(lhs, rhs) => {
        calculate_yell(lhs, monkeys, cache) / calculate_yell(rhs, monkeys, cache)
      }
    };

    cache.insert(monkey, val);
    val
  }

  let mut cache = HashMap::new();
  calculate_yell("root", &yells, &mut cache)
}

pub(crate) fn part_2(input: &str) -> i128 {
  fn calculate_yell(
    idx: &usize,
    monkey: &BTreeMap<usize, Yell>,
    cache: HashMap<usize, i128>,
  ) -> i128 {
    todo!()
  }
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
    assert_eq!(res, 152);
  }

  #[test]
  #[ignore = "later"]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 0)
  }
}
