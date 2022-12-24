use std::{convert::Infallible, fmt::Display, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("input");

#[derive(Debug, Clone, Copy)]
enum Direction {
  Right,
  Left,
  Up,
  Down,
}

impl Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Direction::Right => '>',
        Direction::Left => '<',
        Direction::Up => '^',
        Direction::Down => 'v',
      }
    )
  }
}

impl From<Direction> for usize {
  fn from(value: Direction) -> Self {
    match value {
      Direction::Right => 0,
      Direction::Down => 1,
      Direction::Left => 2,
      Direction::Up => 3,
    }
  }
}

impl From<usize> for Direction {
  fn from(value: usize) -> Self {
    match value {
      0 => Direction::Right,
      1 => Direction::Down,
      2 => Direction::Left,
      3 => Direction::Up,
      _ => unreachable!(),
    }
  }
}

impl Direction {
  fn len() -> usize {
    4
  }

  fn calculate_new_direction(&self, other: &Instruction) -> Direction {
    let value = match other {
      Instruction::Right => (usize::from(*self) + 1) % Direction::len(),
      Instruction::Left => (usize::from(*self) + 3) % Direction::len(),
      _ => usize::from(*self),
    };
    Direction::from(value)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
  Free,
  Blocked,
  Void,
}

impl From<char> for Tile {
  fn from(value: char) -> Self {
    match value {
      ' ' => Self::Void,
      '.' => Self::Free,
      '#' => Self::Blocked,
      _ => unreachable!(),
    }
  }
}

impl Display for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Void => ' ',
        Self::Free => '.',
        Self::Blocked => '#',
      }
    )
  }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
  Right,
  Left,
  Move(usize),
}

#[derive(Debug)]
struct Cove {
  tile_matrix: Vec<Vec<Tile>>,
  current_direction: Direction,
  current_position: [usize; 2],
  moves: Vec<([usize; 2], Direction)>,
}

impl FromStr for Cove {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let input = s.trim_end();

    let rows = input.lines().count();
    let cols = input.lines().map(|row| row.len()).max().unwrap();

    let mut tile_matrix = vec![vec![Tile::Void; cols]; rows];
    input.split("\n").enumerate().for_each(|(y, line)| {
      line
        .chars()
        .enumerate()
        .for_each(|(x, c)| tile_matrix[y][x] = Tile::from(c));
    });

    let current_position = [
      tile_matrix[0]
        .iter()
        .position(|&t| t == Tile::Free)
        .unwrap(),
      0,
    ];

    let current_direction = Direction::Right;

    Ok(Cove {
      tile_matrix,
      current_direction,
      current_position,
      moves: vec![(current_position, current_direction)],
    })
  }
}

impl Display for Cove {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let rows = self.tile_matrix.len();
    let cols = self.tile_matrix.iter().map(|row| row.len()).max().unwrap();

    let mut display = vec![vec![" ".to_string(); cols]; rows];

    for y in 0..rows {
      for x in 0..cols {
        display[y][x] = self.tile_matrix[y][x].to_string()
      }
    }

    for ([x, y], direction) in self.moves.iter() {
      display[*y][*x] = direction.to_string()
    }

    for y in 0..rows {
      for x in 0..cols {
        write!(f, "{}", display[y][x])?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

impl Cove {
  fn calculate_password(&self) -> usize {
    1000 * (self.current_position[1] + 1)
      + 4 * (self.current_position[0] + 1)
      + usize::from(self.current_direction)
  }

  fn calculate_new_position(&self, direction: &Direction, [x, y]: [usize; 2]) -> [usize; 2] {
    let (x_dim, y_dim) = (self.tile_matrix[y].len(), self.tile_matrix.len());
    match direction {
      Direction::Right => [(x + 1) % x_dim, y],
      Direction::Left => [(x + x_dim - 1) % x_dim, y],
      Direction::Up => [x, (y + y_dim - 1) % y_dim],
      Direction::Down => [x, (y + 1) % y_dim],
    }
  }

  fn change_direction(&mut self, instruction: &Instruction) {
    self.current_direction = self.current_direction.calculate_new_direction(instruction);
    self
      .moves
      .push((self.current_position, self.current_direction))
  }

  fn current_direction(&mut self, new_direction: Direction) {
    self.current_direction = new_direction
  }

  fn current_position(&mut self, [x, y]: [usize; 2]) {
    self.current_position = [x, y];
    self
      .moves
      .push((self.current_position, self.current_direction))
  }

  fn change_position(&mut self, amount: usize, wrap: impl Fn(&Self) -> ([usize; 2], Direction)) {
    for _ in 0..amount {
      let [x, y] = self.calculate_new_position(&self.current_direction, self.current_position);
      match self
        .tile_matrix
        .get(y)
        .and_then(|row| row.get(x))
        .unwrap_or(&Tile::Void)
      {
        Tile::Free => self.current_position([x, y]),
        Tile::Blocked => break,
        Tile::Void => {
          let ([new_x, new_y], direction) = wrap(self);
          if self.tile_matrix[new_y][new_x] == Tile::Blocked {
            break;
          }
          self.current_position([new_x, new_y]);
          self.current_direction(direction);
        }
      }
    }
  }

  fn wrap_cube<const N: usize>(&self) -> ([usize; 2], Direction) {
    let [x, y] = self.current_position;

    let (qx, qy, new_direction) = match (x / N, y / N, self.current_direction) {
      (1, 0, Direction::Up) => (0, 3, Direction::Right),
      (1, 0, Direction::Left) => (0, 2, Direction::Right),
      (2, 0, Direction::Up) => (0, 3, Direction::Up),
      (2, 0, Direction::Right) => (1, 2, Direction::Left),
      (2, 0, Direction::Down) => (1, 1, Direction::Left),
      (1, 1, Direction::Right) => (2, 0, Direction::Up),
      (1, 1, Direction::Left) => (0, 2, Direction::Down),
      (0, 2, Direction::Up) => (1, 1, Direction::Right),
      (0, 2, Direction::Left) => (1, 0, Direction::Right),
      (1, 2, Direction::Right) => (2, 0, Direction::Left),
      (1, 2, Direction::Down) => (0, 3, Direction::Left),
      (0, 3, Direction::Right) => (1, 2, Direction::Up),
      (0, 3, Direction::Down) => (2, 0, Direction::Down),
      (0, 3, Direction::Left) => (1, 0, Direction::Down),
      _ => {
        dbg!(y, x, y / N, x / N, self.current_direction);
        unreachable!()
      }
    };

    let (dx, dy) = (x % N, y % N);
    let i = match self.current_direction {
      Direction::Right => dy,
      Direction::Left => N - 1 - dy,
      Direction::Up => dx,
      Direction::Down => N - 1 - dx,
    };

    let (nx, ny) = match new_direction {
      Direction::Right => (0, i),
      Direction::Left => (N - 1, N - 1 - i),
      Direction::Up => (i, N - 1),
      Direction::Down => (N - 1 - i, 0),
    };

    ([qx * N + nx, qy * N + ny], new_direction)
  }

  fn wrap(&self) -> ([usize; 2], Direction) {
    let [mut x, mut y] =
      self.calculate_new_position(&self.current_direction, self.current_position);
    while *self
      .tile_matrix
      .get(y)
      .and_then(|row| row.get(x))
      .unwrap_or(&Tile::Void)
      == Tile::Void
    {
      [x, y] = self.calculate_new_position(&self.current_direction, [x, y]);
    }

    ([x, y], self.current_direction)
  }

  fn walk(
    &mut self,
    instructions: &Vec<Instruction>,
    wrap: &impl Fn(&Self) -> ([usize; 2], Direction),
  ) {
    for instruction in instructions.iter() {
      match instruction {
        Instruction::Right | Instruction::Left => self.change_direction(instruction),
        Instruction::Move(amount) => self.change_position(*amount, wrap),
      }
    }
  }
}

fn parse(input: &str) -> (Cove, Vec<Instruction>) {
  let input = input.trim_end();
  let (map, instructions) = input.split_once("\n\n").unwrap();
  let movements = instructions
    .split_terminator(&['L', 'R'])
    .map(|s| Instruction::Move(s.parse().unwrap()))
    .collect::<Vec<_>>();
  let directions = instructions
    .chars()
    .filter_map(|c| match c {
      'L' => Some(Instruction::Left),
      'R' => Some(Instruction::Right),
      _ => None,
    })
    .collect::<Vec<_>>();

  let mut instructions = vec![];
  for iter in movements.into_iter().zip_longest(directions) {
    match iter {
      itertools::EitherOrBoth::Both(move_, direction) => {
        instructions.extend_from_slice(&[move_, direction])
      }
      itertools::EitherOrBoth::Left(move_) => instructions.push(move_),
      itertools::EitherOrBoth::Right(direction) => instructions.push(direction),
    }
  }
  (Cove::from_str(map).unwrap(), instructions)
}

fn part_1(input: &str) -> usize {
  let (mut cove, instructions) = parse(input);
  cove.walk(&instructions, &Cove::wrap);
  cove.calculate_password()
}

fn part_2<const N: usize>(input: &str) -> usize {
  let (mut cove, instructions) = parse(input);
  cove.walk(&instructions, &Cove::wrap_cube::<N>);
  cove.calculate_password()
}

fn main() {
  println!("Part 1: {}", part_1(INPUT));
  println!("Part 2: {}", part_2::<50>(INPUT));
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = include_str!("sample");

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 6032);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2::<4>(TEST_INPUT);
    assert_eq!(res, 5031)
  }
}
