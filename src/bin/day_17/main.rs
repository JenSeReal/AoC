use std::fmt::Display;

use itertools::Itertools;
use nom::{
  branch::alt,
  character::complete::{self, line_ending},
  multi::{many1, many_m_n, separated_list1},
  IResult, Parser,
};

const INPUT: &str = include_str!("input");
const CHAMBER_WIDTH: usize = 7;
const ROCK_FORMATIONS: &str = include_str!("rock_formations");

type RockFormation = Vec<Vec<Material>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
  Air,
  Rock(State),
}

impl From<char> for Material {
  fn from(value: char) -> Self {
    match value {
      '.' => Material::Air,
      '#' => Material::Rock(State::Stopped),
      _ => unreachable!(),
    }
  }
}

impl From<Material> for char {
  fn from(value: Material) -> Self {
    match value {
      Material::Air => '.',
      Material::Rock(state) => state.into(),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
  Falling,
  Stopped,
}

impl From<State> for char {
  fn from(value: State) -> Self {
    match value {
      State::Falling => '@',
      State::Stopped => '#',
    }
  }
}

#[derive(Debug)]
enum Movement {
  Left,
  Right,
}

impl From<char> for Movement {
  fn from(value: char) -> Self {
    match value {
      '>' => Self::Right,
      '<' => Self::Left,
      _ => unreachable!(),
    }
  }
}

impl From<&Movement> for isize {
  fn from(value: &Movement) -> Self {
    match value {
      Movement::Left => -1,
      Movement::Right => 1,
    }
  }
}

#[derive(Debug, Clone)]
struct Chamber {
  fields: Vec<[Material; CHAMBER_WIDTH]>,
}

impl Default for Chamber {
  fn default() -> Self {
    Self { fields: vec![] }
  }
}

impl Display for Chamber {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for states in self.fields.iter().rev() {
      write!(f, "|")?;
      for (_, state) in states.into_iter().enumerate() {
        write!(f, "{}", char::from(*state))?;
      }
      writeln!(f, "|")?;
    }

    for i in 0..=CHAMBER_WIDTH + 1 {
      let char = if i % (CHAMBER_WIDTH + 1) == 0 {
        '+'
      } else {
        '-'
      };
      write!(f, "{char}")?;
    }

    writeln!(f, "")
  }
}

impl Chamber {
  fn simulate(
    &mut self,
    movements: Vec<Movement>,
    rock_formations: Vec<RockFormation>,
    rock_count: usize,
  ) {
    let mut movements = movements.iter().cycle();
    for mut rock_formation in rock_formations.into_iter().cycle().take(rock_count) {
      let mut ceiling = self.fields.len() + 3 + rock_formation.len();
      loop {
        ceiling -= 1;
        let movement = movements.next().unwrap();
        let no_side_collisions = rock_formation
          .iter()
          .enumerate()
          .map(|(height, row)| (row, self.fields.get(ceiling - height)))
          .all(|(row, chamber_row)| {
            if match movement {
              Movement::Left => !matches!(row[0], Material::Air),
              Movement::Right => !matches!(row[6], Material::Air),
            } {
              return false;
            }
            let Some(chamber_row) = chamber_row else {
                        return true;
                    };
            match movement {
              Movement::Right => row[0..6]
                .iter()
                .zip(chamber_row[1..7].iter())
                .all(|(a, b)| matches!(a, Material::Air) || matches!(b, Material::Air)),
              Movement::Left => row[1..7]
                .iter()
                .zip(chamber_row[0..6].iter())
                .all(|(a, b)| matches!(a, Material::Air) || matches!(b, Material::Air)),
            }
          });
        if no_side_collisions {
          rock_formation.iter_mut().for_each(|row| {
            row.rotate_right(match movement {
              Movement::Left => 6,
              Movement::Right => 1,
            });
          });
        }
        if ceiling < rock_formation.len() {
          break;
        }
        let no_down_collisions = rock_formation.iter().enumerate().all(|(row_idx, row)| {
          if let Some(chamber_row) = self.fields.get(ceiling - row_idx - 1) {
            row
              .iter()
              .zip(chamber_row.iter())
              .all(|(a, b)| matches!(a, Material::Air) || matches!(b, Material::Air))
          } else {
            true
          }
        });
        if !no_down_collisions {
          break;
        }
      }
      self
        .fields
        .resize(self.fields.len().max(ceiling + 1), [Material::Air; 7]);
      for (row_idx, row) in rock_formation.into_iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
          if !matches!(cell, Material::Air) {
            self.fields[ceiling - row_idx][x] = *cell;
          }
        }
      }
    }
  }
}

fn parse_rock(input: &str) -> IResult<&str, Vec<Material>> {
  many1(alt((
    complete::char('#').map(|_| Material::Rock(State::Falling)),
    complete::char('.').map(|_| Material::Air),
  )))(input)
}

fn parse_formation(input: &str) -> IResult<&str, RockFormation> {
  let (input, formation) = separated_list1(line_ending, parse_rock)(input)?;
  let mut filled_formation = vec![vec![Material::Air; 7]; formation.len()];

  for (row, rock_row) in formation.iter().enumerate() {
    for (col, material) in rock_row.iter().enumerate() {
      filled_formation[row][col + 2] = *material
    }
  }

  Ok((input, filled_formation))
}

fn parse(input: &str) -> (Vec<Movement>, Vec<RockFormation>) {
  let movements = input.trim().chars().map(Into::into).collect();
  let rock_formations =
    separated_list1(many_m_n(2, 2, line_ending), parse_formation)(ROCK_FORMATIONS)
      .unwrap()
      .1;

  (movements, rock_formations)
}

fn count_rocks_in_chamber_slice(slice: &[[Material; 7]]) -> usize {
  slice
    .iter()
    .flatten()
    .filter(|&&cell| !matches!(cell, Material::Air))
    .count()
    * 5
    / 22
}

pub(crate) fn part_1(input: &str) -> usize {
  let (movements, rock_formations) = parse(input);
  let mut chamber = Chamber::default();

  chamber.simulate(movements, rock_formations, 2022);
  chamber.fields.len()
}

pub(crate) fn part_2(input: &str) -> u64 {
  let (movements, rock_formations) = parse(input);
  let mut chamber = Chamber::default();

  chamber.simulate(movements, rock_formations, 5000);
  let (pattern_start, pattern_length) = chamber
    .fields
    .windows(50)
    .enumerate()
    .tuple_combinations()
    .find(|((_, a), (_, b))| a == b)
    .map(|((i, _), (j, _))| (i, j - i))
    .expect("There should be a pattern!");

  let rocks_before_pattern = count_rocks_in_chamber_slice(&chamber.fields[..pattern_start]);

  let rocks_generated_in_pattern =
    count_rocks_in_chamber_slice(&chamber.fields[pattern_start..pattern_start + pattern_length]);
  let num_pattern_repetitions =
    (1_000_000_000_000 - rocks_before_pattern as u64) / rocks_generated_in_pattern as u64;
  let leftover_rocks =
    (1_000_000_000_000 - rocks_before_pattern as u64) % rocks_generated_in_pattern as u64;
  let leftover_rocks_height = (0..=pattern_length)
    .find(|&i| {
      count_rocks_in_chamber_slice(&chamber.fields[pattern_start..pattern_start + i])
        >= leftover_rocks as usize
    })
    .expect("There should be a leftover rock height");
  num_pattern_repetitions * pattern_length as u64
    + pattern_start as u64
    + leftover_rocks_height as u64
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
    assert_eq!(res, 3068);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 1514285714288)
  }
}
