#![allow(unused_variables)]

use std::{collections::HashSet, convert::Infallible, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("input");

const SIDES: [[i32; 3]; 6] = [
  [-1, 0, 0],
  [1, 0, 0],
  [0, -1, 0],
  [0, 1, 0],
  [0, 0, -1],
  [0, 0, 1],
];

#[derive(Debug)]
struct Grid<T> {
  coords: Vec<[T; 3]>,
  seen: HashSet<[T; 3]>,
  stack: Vec<[T; 3]>,
}

impl FromStr for Grid<i32> {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      coords: s
        .trim()
        .lines()
        .filter_map(|l| {
          l.split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .and_then(|(x, y, z)| Some([x, y, z]))
        })
        .collect_vec(),
      seen: HashSet::new(),
      stack: vec![[0; 3]],
    })
  }
}

impl Grid<i32> {
  fn unique(&self) -> HashSet<[i32; 3]> {
    HashSet::from_iter(self.coords.iter().cloned())
  }

  fn sides([x, y, z]: [i32; 3]) -> [[i32; 3]; 6] {
    SIDES
      .iter()
      .map(|[dx, dy, dz]| [x + dx, y + dy, z + dz])
      .collect::<Vec<_>>()
      .try_into()
      .unwrap()
  }

  fn solve(&mut self) -> HashSet<[i32; 3]> {
    let drops = self.unique();
    let max = drops.iter().flatten().max().unwrap() + 1;

    while let Some([x, y, z]) = self.stack.pop() {
      for [x, y, z] in Self::sides([x, y, z]) {
        if !drops.contains(&[x, y, z])
          && !self.seen.contains(&[x, y, z])
          && [x, y, z].into_iter().all(|i| (-1..=max).contains(&i))
        {
          self.seen.insert([x, y, z]);
          self.stack.push([x, y, z])
        }
      }
    }
    drops
  }
}

pub(crate) fn part_1(input: &str) -> usize {
  let mut grid = Grid::from_str(input).unwrap();
  let droplets = grid.solve();

  droplets
    .iter()
    .flat_map(|p| Grid::sides(*p))
    .filter(|&[x, y, z]| !droplets.contains(&[x, y, z]))
    .count()
}

pub(crate) fn part_2(input: &str) -> usize {
  let mut grid = Grid::from_str(input).unwrap();
  let droplets = grid.solve();

  droplets
    .iter()
    .flat_map(|&p| Grid::sides(p))
    .filter(|s| grid.seen.contains(s))
    .count()
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
    assert_eq!(res, 64);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 58)
  }
}
