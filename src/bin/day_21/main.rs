#![allow(unused_variables)]

use std::{
  cmp::Ordering,
  collections::{BTreeMap, HashMap},
};

const INPUT: &str = include_str!("input");

#[derive(Debug, Clone)]
enum Yell<'a> {
  Number(i128),
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

fn evaluate<'a>(
  monkey: &'a str,
  yells: &'a BTreeMap<&'a str, Yell>,
  cache: &mut HashMap<&'a str, i128>,
) -> i128 {
  if let Some(val) = cache.get(monkey) {
    return *val;
  };

  let val = match yells.get(monkey).unwrap() {
    Yell::Number(val) => *val,
    Yell::Add(lhs, rhs) => evaluate(lhs, yells, cache) + evaluate(rhs, yells, cache),
    Yell::Sub(lhs, rhs) => evaluate(lhs, yells, cache) - evaluate(rhs, yells, cache),
    Yell::Mul(lhs, rhs) => evaluate(lhs, yells, cache) * evaluate(rhs, yells, cache),
    Yell::Div(lhs, rhs) => evaluate(lhs, yells, cache) / evaluate(rhs, yells, cache),
  };

  cache.insert(monkey, val);
  val
}

pub(crate) fn part_1(input: &str) -> i128 {
  let yells = parse(input);
  let mut cache = HashMap::new();
  evaluate("root", &yells, &mut cache)
}

fn binary_search<F>(mut lo: i128, mut hi: i128, mut cmp: F) -> Option<i128>
where
  F: FnMut(i128) -> Ordering,
{
  while lo != hi {
    let mid = (lo + hi) / 2;
    match cmp(mid) {
      Ordering::Equal => hi = mid,
      Ordering::Less => hi = mid - 1,
      Ordering::Greater => lo = mid + 1,
    }
  }
  match cmp(lo) {
    Ordering::Equal => Some(lo),
    Ordering::Less => None,
    Ordering::Greater => None,
  }
}

pub(crate) fn part_2(input: &str) -> i128 {
  let mut yells = parse(input);
  let mut cache = HashMap::new();

  let (lhs, rhs) = match *yells.get("root").unwrap() {
    Yell::Add(lhs, rhs) => (lhs, rhs),
    Yell::Sub(lhs, rhs) => (lhs, rhs),
    Yell::Mul(lhs, rhs) => (lhs, rhs),
    Yell::Div(lhs, rhs) => (lhs, rhs),
    _ => unreachable!(),
  };

  let start = evaluate(lhs, &yells, &mut cache).cmp(&evaluate(rhs, &yells, &mut cache));
  binary_search(0, i64::MAX as i128, move |x| {
    let mut cache = HashMap::new();
    yells.insert("humn", Yell::Number(x));
    let next = evaluate(lhs, &yells, &mut cache).cmp(&evaluate(rhs, &yells, &mut cache));
    match next {
      Ordering::Equal => Ordering::Equal,
      next => match start == next {
        true => Ordering::Greater,
        false => Ordering::Less,
      },
    }
  })
  .unwrap()
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
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 301)
  }
}
