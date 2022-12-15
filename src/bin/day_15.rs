use std::{collections::BTreeMap, fmt::Display};
use std::{collections::HashSet, fs::read_to_string};

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
  covered: Vec<[isize; 2]>,
}

impl Grid {
  fn distance(sensor: &(isize, isize), beacon: &(isize, isize)) -> usize {
    ((sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()) as usize
  }
}

impl From<BTreeMap<(isize, isize), (isize, isize)>> for Grid {
  fn from(grid: BTreeMap<(isize, isize), (isize, isize)>) -> Self {
    Self {
      grid,
      covered: vec![],
    }
  }
}

impl Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let grid_x_min = self
      .grid
      .iter()
      .map(|(sensor, beacon)| sensor.0.min(beacon.0))
      .min();

    let grid_x_max = self
      .grid
      .iter()
      .map(|(sensor, beacon)| sensor.0.max(beacon.0))
      .max();

    let grid_y_min = self
      .grid
      .iter()
      .map(|(sensor, beacon)| sensor.1.min(beacon.1))
      .min();

    let grid_y_max = self
      .grid
      .iter()
      .map(|(sensor, beacon)| sensor.1.max(beacon.1))
      .max();

    let x_covered_min = self.covered.iter().map(|[x, _]| x).min();
    let x_covered_max = self.covered.iter().map(|[x, _]| x).max();
    let y_covered_min = self.covered.iter().map(|[_, y]| y).min();
    let y_covered_max = self.covered.iter().map(|[_, y]| y).max();

    let x_correction = match (grid_x_min, x_covered_min) {
      (None, Some(x)) => *x,
      (Some(x), None) => x,
      (Some(x), Some(other)) => x.min(*other),
      _ => 0,
    } * -1;
    let y_correction = match (grid_y_min, x_covered_min) {
      (None, Some(x)) => *x,
      (Some(x), None) => x,
      (Some(x), Some(other)) => x.min(*other),
      _ => 0,
    } * -1;

    let x_min = match (grid_x_min, x_covered_min) {
      (None, Some(x)) => *x,
      (Some(x), None) => x,
      (Some(x), Some(other)) => x.min(*other),
      _ => 0,
    };

    let x_max = match (grid_x_max, x_covered_max) {
      (None, Some(x)) => *x,
      (Some(x), None) => x,
      (Some(x), Some(other)) => x.max(*other),
      _ => 0,
    };

    let y_min = match (grid_y_min, y_covered_min) {
      (None, Some(y)) => *y,
      (Some(y), None) => y,
      (Some(y), Some(other)) => y.min(*other),
      _ => 0,
    };

    let y_max = match (grid_y_max, y_covered_max) {
      (None, Some(y)) => *y,
      (Some(y), None) => y,
      (Some(y), Some(other)) => y.max(*other),
      _ => 0,
    };

    let mut grid =
      vec![vec![State::Air; (x_max - x_min) as usize + 1]; (y_max - y_min) as usize + 1];

    self.grid.iter().for_each(|(sensor, beacon)| {
      grid[(sensor.1 + y_correction) as usize][(sensor.0 + x_correction) as usize] = State::Sensor;
      grid[(beacon.1 + y_correction) as usize][(beacon.0 + x_correction) as usize] = State::Beacon;
    });

    self.covered.iter().enumerate().for_each(|(row, [x, y])| {
      let x = (x + x_correction) as usize;
      let y = (y + y_correction) as usize;
      for i in x..=y {
        match grid[row][i] {
          State::Air => grid[row][i] = State::Covered,
          _ => (),
        }
      }
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
  let mut known_beacon = HashSet::new();

  for (sensor, beacon) in grid.grid.iter() {
    let distance = Grid::distance(sensor, beacon) as isize;

    let offset = distance - (sensor.1 - row as isize).abs();
    // ignore sensor
    if offset < 0 {
      continue;
    }

    let x_min = sensor.0 - offset;
    let x_max = sensor.0 + offset;

    grid.covered.push([x_min, x_max]);

    if beacon.1 == row as isize {
      known_beacon.insert(beacon.0);
    }
  }

  grid.covered.sort();

  let mut merged = vec![];

  for [min, max] in grid.covered.into_iter() {
    if merged.is_empty() {
      merged.push([min, max]);
      continue;
    }

    let [_, merged_max] = merged.last().unwrap().clone();

    if min > merged_max + 1 {
      merged.push([min, max]);
      continue;
    }

    merged.last_mut().unwrap()[1] = merged_max.max(max);
  }

  let mut no_beacon = HashSet::new();

  for [min, max] in merged {
    for i in min..=max {
      no_beacon.insert(i);
    }
  }

  no_beacon.len() - known_beacon.len()
}

pub(crate) fn part_2(input: &str, tuning_frequencies: usize) -> usize {
  let mut grid = parse(input);
  for test_frequency in 0..=tuning_frequencies {
    println!("{test_frequency}");
    grid.covered.clear();
    for (sensor, beacon) in grid.grid.iter() {
      let distance = Grid::distance(sensor, beacon) as isize;

      let offset = distance - (sensor.1 - test_frequency as isize).abs();
      // ignore sensor
      if offset < 0 {
        continue;
      }

      let x_min = sensor.0 - offset;
      let x_max = sensor.0 + offset;

      grid.covered.push([x_min, x_max]);
    }

    grid.covered.sort();

    let mut merged = vec![];

    for [min, max] in grid.covered.iter() {
      if merged.is_empty() {
        merged.push([*min, *max]);
        continue;
      }

      let [_, merged_max] = merged.last().unwrap().clone();

      if *min > merged_max + 1 {
        merged.push([*min, *max]);
        continue;
      }

      merged.last_mut().unwrap()[1] = merged_max.max(*max);
    }

    let mut x = 0;
    for [min, max] in merged {
      if x < min {
        return x as usize * 4_000_000 + test_frequency;
      } else {
        x = max + 1
      }
      if x > tuning_frequencies as isize {
        break;
      }
    }
  }

  todo!()
}

fn main() {
  let input = read_to_string("assets/day_15").unwrap();
  let part_1 = part_1(&input, 2_000_000);
  println!("Part 1: {}", part_1);
  let part_2 = part_2(&input, 4_000_000);
  println!("Part 2: {}", part_2);
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
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT, 20);
    assert_eq!(res, 56000011)
  }
}
