use std::{collections::HashSet, convert::Infallible, fmt::Display, str::FromStr};

const INPUT: &str = include_str!("input");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl TryFrom<char> for Direction {
  type Error = ();

  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '^' => Ok(Self::Up),
      '>' => Ok(Self::Right),
      'v' => Ok(Self::Down),
      '<' => Ok(Self::Left),
      _ => Err(()),
    }
  }
}

impl From<Direction> for usize {
  fn from(value: Direction) -> Self {
    match value {
      Direction::Up => 2,
      Direction::Down => 3,
      Direction::Left => 0,
      Direction::Right => 1,
    }
  }
}

impl From<Direction> for char {
  fn from(value: Direction) -> Self {
    match value {
      Direction::Up => '^',
      Direction::Down => 'v',
      Direction::Left => '<',
      Direction::Right => '>',
    }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Tile {
  Wall,
  Free,
  Direction(Direction),
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Tile::Wall => '#',
        Tile::Free => '.',
        Tile::Direction(dir) => char::from(*dir),
      }
    )
  }
}

impl From<char> for Tile {
  fn from(value: char) -> Self {
    match value {
      '#' => Self::Wall,
      '.' => Self::Free,
      '^' | '>' | 'v' | '<' => Tile::Direction(Direction::try_from(value).unwrap()),
      _ => unreachable!(),
    }
  }
}

#[derive(Debug)]
struct Valley {
  rows: usize,
  cols: usize,
  blizzards: Vec<(usize, usize, Direction)>,
  positions: HashSet<(usize, usize)>,
  minute: usize,
}

impl FromStr for Valley {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let rows = s.lines().count();
    let cols = s.lines().next().unwrap().len();

    let blizzards = s
      .lines()
      .enumerate()
      .flat_map(|(x, row)| {
        row
          .chars()
          .enumerate()
          .filter_map(move |(y, c)| Some((x, y, Direction::try_from(c).ok()?)))
      })
      .collect::<Vec<_>>();

    Ok(Self {
      rows,
      cols,
      blizzards,
      positions: HashSet::from([(0, 1)]),
      minute: 0,
    })
  }
}

impl Valley {
  fn traverse(&mut self, target: &(usize, usize)) {
    for minutes in 1.. {
      for (y, x, d) in self.blizzards.iter_mut() {
        match d {
          Direction::Right => *x = if *x == self.cols - 2 { 1 } else { *x + 1 },
          Direction::Left => *x = if *x == 1 { self.cols - 2 } else { *x - 1 },
          Direction::Down => *y = if *y == self.rows - 2 { 1 } else { *y + 1 },
          Direction::Up => *y = if *y == 1 { self.rows - 2 } else { *y - 1 },
        }
      }
      let bpos = self
        .blizzards
        .iter()
        .map(|&(x, y, _)| (x, y))
        .collect::<HashSet<_>>();
      let mut next_positions = HashSet::with_capacity(self.positions.len());
      for &(x, y) in &self.positions {
        for (dx, dy) in [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)] {
          if (x == 0 && dx == -1) || (x == self.rows - 1 && dx == 1) {
            continue;
          }
          let (x, y) = (x as i32 + dx, y as i32 + dy);
          if (x != 0 || y == 1)
            && (x as usize != self.rows - 1 || y as usize == self.cols - 2)
            && y != 0
            && y as usize != self.cols - 1
            && !bpos.contains(&(x as usize, y as usize))
          {
            next_positions.insert((x as usize, y as usize));
          }
        }
      }
      self.positions = next_positions;
      if self.positions.contains(target) {
        self.minute += minutes;
        break;
      }
    }
  }
}

pub(crate) fn part_1(input: &str) -> usize {
  let mut valley = Valley::from_str(input).unwrap();

  valley.traverse(&(valley.rows - 1, valley.cols - 2));

  valley.minute
}
pub(crate) fn part_2(input: &str) -> usize {
  let mut valley = Valley::from_str(input).unwrap();

  valley.traverse(&(valley.rows - 1, valley.cols - 2));
  valley.positions = HashSet::from_iter([(valley.rows - 1, valley.cols - 2)]);
  valley.traverse(&(0, 1));
  valley.positions = HashSet::from_iter([(0, 1)]);
  valley.traverse(&(valley.rows - 1, valley.cols - 2));

  valley.minute
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
    assert_eq!(res, 18);
  }

  #[test]
  #[ignore = "later"]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 54)
  }
}
