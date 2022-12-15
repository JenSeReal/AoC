use std::fs::read_to_string;
#[allow(dead_code)]
use std::{collections::BTreeMap, fmt::Display};

use nom::{
  bytes::complete::tag,
  character::complete,
  combinator::{all_consuming, map},
  sequence::{preceded, separated_pair},
  Finish, IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
  Covered,
  Air,
  Sensor,
  Beacon,
}

impl Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        State::Covered => '#',
        State::Air => '.',
        State::Sensor => 'S',
        State::Beacon => 'B',
      }
    )
  }
}

impl Default for State {
  fn default() -> Self {
    Self::Covered
  }
}

#[derive(Debug, Clone)]
struct Grid {
  grid: BTreeMap<(isize, isize), (isize, isize)>,
  covered: BTreeMap<(isize, isize), State>,
}

impl Grid {
  fn distance(sensor: &(isize, isize), beacon: &(isize, isize)) -> usize {
    ((sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()) as usize
  }
}

impl From<BTreeMap<(isize, isize), (isize, isize)>> for Grid {
  fn from(grid: BTreeMap<(isize, isize), (isize, isize)>) -> Self {
    let covered = grid
      .iter()
      .fold(BTreeMap::new(), |mut covered, (sensor, beacon)| {
        covered.insert(*sensor, State::Sensor);
        covered.insert(*beacon, State::Beacon);

        covered
      });
    Self { grid, covered }
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let x_min = self.covered.iter().map(|((x, _), _)| x).min().unwrap();
    let x_max = self.covered.iter().map(|((x, _), _)| x).max().unwrap();

    let y_min = self.covered.iter().map(|((_, y), _)| y).min().unwrap();
    let y_max = self.covered.iter().map(|((_, y), _)| y).max().unwrap();

    let x_correction = x_min * -1;
    let y_correction = y_min * -1;

    let mut grid =
      vec![vec![State::Air; (x_max - x_min) as usize + 1]; (y_max - y_min) as usize + 1];

    self.covered.iter().for_each(|((x, y), state)| {
      grid[(y + y_correction) as usize][(x + x_correction) as usize] = state.clone();
    });

    for row in grid.iter() {
      for col in row.iter() {
        write!(f, "{}", col).unwrap();
      }
      writeln!(f).unwrap();
    }

    Ok(())
  }
}

fn parse_coordinate(input: &str) -> IResult<&str, (isize, isize)> {
  map(
    separated_pair(
      preceded(tag("x="), complete::i64),
      tag(", "),
      preceded(tag("y="), complete::i64),
    ),
    |(x, y)| (x as isize, y as isize),
  )(input)
}

fn parse_line(input: &str) -> IResult<&str, ((isize, isize), (isize, isize))> {
  separated_pair(
    preceded(tag("Sensor at "), parse_coordinate),
    tag(": "),
    preceded(tag("closest beacon is at "), parse_coordinate),
  )(input)
}

fn parse(input: &str) -> Grid {
  input
    .trim()
    .lines()
    .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
    .fold(BTreeMap::new(), |mut grid, (sensor, beacon)| {
      grid.insert(sensor, beacon);
      grid
    })
    .into()
}

pub(crate) fn part_1(input: &str, row: usize) -> usize {
  let mut grid = parse(input);

  for (sensor, beacon) in grid.grid.iter() {
    let start_x = sensor.0;
    let start_y = sensor.1;
    let distance = Grid::distance(sensor, beacon) as isize;

    for i in 0..=distance {
      let mut amount = distance;

      for j in 0..=amount - i {
        grid.covered.entry((start_x + j, start_y - i)).or_default();
        grid.covered.entry((start_x - j, start_y - i)).or_default();
        grid.covered.entry((start_x + j, start_y + i)).or_default();
        grid.covered.entry((start_x - j, start_y + i)).or_default();
        amount -= 1
      }
    }
  }

  grid
    .covered
    .into_iter()
    .filter(|((_, y), state)| *y == row as isize && *state == State::Covered)
    .count()
}

pub(crate) fn part_2(input: &str) -> u32 {
  todo!()
}

fn main() {
  let input = read_to_string("assets/day_15").unwrap();
  let part_1 = part_1(&input, 2_000_000);
  println!("{}", part_1);
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3

";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT, 10);
    assert_eq!(res, 26);
  }

  #[test]
  #[ignore = "later"]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 0)
  }
}
