use std::{
  collections::{BTreeMap, VecDeque},
  fmt::Display,
};

use nom::{
  character::complete::newline, combinator::all_consuming, multi::separated_list1, IResult,
};

const START: (usize, usize) = (500, 0);
const X_MAX: usize = 1000;

enum Direction {
  Left,
  Right,
  Down,
}

impl Direction {
  fn as_number(&self) -> isize {
    match self {
      Direction::Left => -1,
      Direction::Right => 1,
      Direction::Down => 0,
    }
  }

  fn as_slice() -> [isize; 3] {
    [0, -1, 1]
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Material {
  Air,
  Rock,
  Sand,
  SandSource,
}

impl Display for Material {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Material::Air => write!(f, "."),
      Material::Rock => write!(f, "#"),
      Material::Sand => write!(f, "o"),
      Material::SandSource => write!(f, "+"),
    }
  }
}

fn parse(input: &str) -> ((usize, usize), usize, Vec<Vec<Material>>) {
  let input = input.trim();
  let rows = input.lines().count();
  let rock_formations =
    input
      .lines()
      .enumerate()
      .fold(vec![vec![]; rows], |mut grid, (row_idx, line)| {
        grid[row_idx] = line
          .split(" -> ")
          .map(|coordinate| {
            coordinate
              .split_once(",")
              .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
              .unwrap()
          })
          .collect::<Vec<_>>();

        grid
      });

  let x_min = rock_formations
    .iter()
    .flatten()
    .map(|(x, y)| x)
    .min()
    .unwrap();

  let x_max = rock_formations
    .iter()
    .flatten()
    .map(|(x, y)| x)
    .max()
    .unwrap();

  let y_max = rock_formations
    .iter()
    .flatten()
    .map(|(x, y)| y)
    .max()
    .unwrap();

  (
    (*x_min, *x_max),
    *y_max,
    rock_formations.iter().fold(
      vec![vec![Material::Air; X_MAX]; *y_max + 1],
      |mut grid, rock_formation| {
        let mut iter = rock_formation.iter().peekable();
        while let Some((x, y)) = iter.next() {
          match iter.peek() {
            Some((next_x, next_y)) => {
              for current_x in usize::min(*x, *next_x)..=usize::max(*x, *next_x) {
                grid[*y][current_x] = Material::Rock
              }
              for current_y in usize::min(*y, *next_y)..=usize::max(*y, *next_y) {
                grid[current_y][*x] = Material::Rock
              }
            }
            None => (),
          }
        }
        grid
      },
    ),
  )
}

fn print(grid: &Vec<Vec<Material>>) {
  for row in grid {
    for col in row {
      print!("{col}");
    }
    println!();
  }
  println!();
}

pub(crate) fn part_1(input: &str) -> u32 {
  let ((x_min, x_max), y_max, mut grid) = parse(input);
  let mut resting = 0;
  grid[0][500] = Material::SandSource;

  'simulation: loop {
    let (mut x, mut y) = (500, 0);
    'sand_drop: loop {
      // abyss
      if y + 1 > y_max {
        break 'simulation;
      }

      for dx in Direction::as_slice() {
        let new_x = (x as isize + dx) as usize;

        if x == 0 || x == x_max {
          break 'simulation;
        }

        // check if bottom left is air
        if grid[y + 1][new_x] == Material::Air {
          x = new_x;
          y += 1;
          continue 'sand_drop;
        }
      }

      grid[y][x] = Material::Sand;
      resting += 1;
      break 'sand_drop;
    }
  }
  resting
}

pub(crate) fn part_2(input: &str) -> u32 {
  let ((x_min, x_max), y_max, mut grid) = parse(input);
  let floor = y_max + 2;
  grid.push(vec![Material::Air; X_MAX]);
  grid.push(vec![Material::Rock; X_MAX]);
  let mut resting = 0;
  let mut queue = VecDeque::new();
  queue.push_back((500, 0));

  while let Some((x, y)) = queue.pop_front() {
    if grid[y][x] == Material::Air {
      resting += 1;
      grid[y][x] = Material::Sand;

      let next_y = y + 1;
      if next_y > floor {
        continue;
      }

      for dx in Direction::as_slice() {
        let next_x = (x as isize + dx as isize) as usize;
        queue.push_back((next_x, next_y))
      }
    }
  }

  resting
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9

";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 24);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 93)
  }
}
