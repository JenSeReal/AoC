#![allow(unused_variables)]

use std::{
  collections::{BTreeSet, HashMap},
  convert::Infallible,
  fmt::Display,
  hash::Hash,
  str::FromStr,
};

use self::Direction::*;

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

const INPUT: &str = include_str!("input");

#[derive(Debug, Clone, Copy)]
enum Tile {
  Elve,
  Ground,
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Tile::Elve => "#",
        Tile::Ground => ".",
      },
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
  NorthWest,
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
}

impl From<Direction> for (isize, isize) {
  fn from(value: Direction) -> Self {
    match value {
      Direction::North => (0, -1),
      Direction::South => (0, 1),
      Direction::West => (-1, 0),
      Direction::East => (1, 0),
      Direction::NorthEast => (1, -1),
      Direction::NorthWest => (-1, -1),
      Direction::SouthEast => (1, 1),
      Direction::SouthWest => (-1, 1),
    }
  }
}

impl Direction {
  fn calculate(&self, [x, y]: &[i32; 2]) -> [i32; 2] {
    match self {
      Direction::North => [*x, y - 1],
      Direction::South => [*x, y + 1],
      Direction::West => [x - 1, *y],
      Direction::East => [x + 1, *y],
      Direction::NorthEast => [x + 1, y - 1],
      Direction::NorthWest => [x - 1, y - 1],
      Direction::SouthEast => [x + 1, y + 1],
      Direction::SouthWest => [x - 1, y + 1],
    }
  }
}

#[derive(Debug)]
struct Grove(BTreeSet<[i32; 2]>);

impl FromStr for Grove {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self(
      s.lines()
        .enumerate()
        .flat_map(|(y, l)| {
          l.chars()
            .enumerate()
            .filter_map(move |(x, c)| (c == '#').then_some([x as i32, y as i32]))
        })
        .collect::<BTreeSet<_>>(),
    ))
  }
}

impl Display for Grove {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let x_min = self.0.iter().map(|[x, _]| x).min().unwrap();
    let x_max = self.0.iter().map(|[x, _]| x).max().unwrap();
    let y_min = self.0.iter().map(|[_, y]| y).min().unwrap();
    let y_max = self.0.iter().map(|[_, y]| y).max().unwrap();

    let x_diff = x_max - x_min;
    let y_diff = y_max - y_min;

    let x_correction = 0 - x_min;
    let y_correction = 0 - y_min;

    let mut grove = vec![vec![Tile::Ground; x_diff as usize + 1]; y_diff as usize + 1];

    self.0.iter().for_each(|&[x, y]| {
      grove[(y + y_correction) as usize][(x + x_correction) as usize] = Tile::Elve
    });

    for row in grove.iter() {
      for tile in row.iter() {
        write!(f, "{}", tile)?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

impl Grove {
  fn proposals(&self, t: usize) -> HashMap<[i32; 2], Vec<[i32; 2]>> {
    let mut proposals = HashMap::<_, Vec<_>>::new();
    for &[x, y] in &self.0 {
      let se = Direction::iter()
        .map(|d| (d, self.0.contains(&d.calculate(&[x, y]))))
        .collect::<HashMap<_, _>>();

      if se.iter().all(|(_, b)| !b) {
        continue;
      }

      let poposal = [
        (North, !se[&North] && !se[&NorthEast] && !se[&NorthWest]),
        (South, !se[&South] && !se[&SouthEast] && !se[&SouthWest]),
        (West, !se[&West] && !se[&NorthWest] && !se[&SouthWest]),
        (East, !se[&East] && !se[&NorthEast] && !se[&SouthEast]),
      ];

      for (i, d) in [North, South, West, East].iter().enumerate() {
        let (d, free) = poposal[(t + i) % 4];
        if free {
          proposals
            .entry(d.calculate(&[x, y]))
            .or_default()
            .push([x, y]);
          break;
        }
      }
    }
    proposals
  }
}

pub(crate) fn part_1(input: &str) -> usize {
  let mut grove = Grove::from_str(input.trim()).unwrap();

  for t in 0..10 {
    let proposals = grove.proposals(t);

    for (pos, proposal) in proposals {
      if proposal.len() == 1 {
        grove.0.remove(&proposal[0]);
        grove.0.insert(pos);
      }
    }
  }
  let (&minx, &maxx) = grove
    .0
    .iter()
    .map(|[x, _]| x)
    .minmax()
    .into_option()
    .unwrap();
  let (&miny, &maxy) = grove
    .0
    .iter()
    .map(|[_, y]| y)
    .minmax()
    .into_option()
    .unwrap();
  (minx..=maxx)
    .cartesian_product(miny..=maxy)
    .filter(|(x, y)| !grove.0.contains(&[*x, *y]))
    .count()
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
    assert_eq!(res, 110);
  }

  #[test]
  #[ignore = "later"]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 0)
  }
}
