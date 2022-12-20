use nom::{
  branch::alt,
  character::complete::{self, line_ending},
  combinator::map,
  multi::{many1, many_m_n, separated_list1},
  IResult,
};

const INPUT: &str = include_str!("input");
const CHAMBER_WIDTH: usize = 7;
const ROCK_FORMATIONS: &str = include_str!("rock_formations");

type RockFormation = [[Material; 4]; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
  Air,
  Rock,
}

impl From<char> for Material {
  fn from(value: char) -> Self {
    match value {
      '#' => Material::Rock,
      '.' => Material::Air,
      _ => unreachable!(),
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
  fields: Vec<[Material; CHAMBER_WIDTH + 2]>,
  ceiling: usize,
  moves: Vec<(usize, usize, isize, usize)>,
  heights: Vec<usize>,
  to_add: Option<usize>,
}

impl Default for Chamber {
  fn default() -> Self {
    Self {
      fields: vec![[Material::Rock; CHAMBER_WIDTH + 2]; 1],
      ceiling: 0,
      moves: vec![],
      heights: vec![],
      to_add: None,
    }
  }
}

impl Chamber {
  fn add_rows(&mut self) {
    self.fields.resize(
      self.ceiling + 8,
      [
        Material::Rock,
        Material::Air,
        Material::Air,
        Material::Air,
        Material::Air,
        Material::Air,
        Material::Air,
        Material::Air,
        Material::Rock,
      ],
    );
  }

  fn add_move(&mut self, value: (usize, usize, isize, usize)) {
    self.moves.push(value)
  }

  fn add_height(&mut self, value: usize) {
    self.heights.push(value)
  }

  fn collision(&self, rock_formation: &RockFormation, x: usize, y: usize) -> bool {
    for dy in 0..4 {
      for dx in 0..4 {
        if rock_formation[dy][dx] == Material::Rock && self.fields[y + dy][x + dx] == Material::Rock
        {
          return true;
        }
      }
    }
    false
  }

  fn find_repeating(&self) -> Option<usize> {
    let len = self.moves.len();

    for sub_len in 1.max(len / 3)..len / 2 {
      if self.moves[len - sub_len * 2..len - sub_len].eq(&self.moves[len - sub_len..]) {
        return Some(sub_len);
      }
    }
    None
  }

  fn solve(
    &mut self,
    rock_formations: &Vec<RockFormation>,
    movements: &Vec<Movement>,
    amount: usize,
  ) {
    let mut rock_formations = rock_formations.iter().enumerate().cycle();
    let mut movements = movements.iter().enumerate().cycle();

    let mut i = 0;
    while i < amount {
      i += 1;
      let (kind, rock_formation) = rock_formations.next().unwrap();
      self.add_rows();

      let start = self.ceiling + 3 + 1;
      let mut y = start;
      let mut x = 3isize;
      let (mut movement_idx, mut movement);
      loop {
        (movement_idx, movement) = movements.next().unwrap();
        let new_x = x + isize::from(movement);
        if !self.collision(rock_formation, new_x as usize, y) {
          x = new_x;
        }
        let new_y = y - 1;
        if self.collision(rock_formation, x as usize, new_y) {
          break;
        } else {
          y = new_y;
        }
      }

      for dy in 0..4 {
        for dx in 0..4 {
          if rock_formation[dy][dx] == Material::Rock {
            self.fields[y + dy][x as usize + dx] = Material::Rock;
            self.ceiling = self.ceiling.max(y + dy);
          }
        }
      }

      if self.to_add.is_none() {
        self.add_move((kind, movement_idx, x, start - y));
        if let Some(len) = self.find_repeating() {
          let rocks_left = amount - i;
          let height_diff = self.ceiling - self.heights[self.heights.len() - len];
          let batches = rocks_left / len;
          self.to_add = Some(height_diff * batches);
          i += batches * len;
        }
        self.add_height(self.ceiling);
      }
    }
  }
}

fn parse_movement(input: &str) -> Vec<Movement> {
  input.trim().chars().map(Into::into).collect()
}

fn parse_rock(input: &str) -> IResult<&str, Vec<Material>> {
  many1(map(
    alt((complete::char('.'), complete::char('#'))),
    Into::into,
  ))(input)
}

fn parse_formation(input: &str) -> IResult<&str, RockFormation> {
  let (input, formation) = separated_list1(line_ending, parse_rock)(input)?;
  let mut filled_formation = [[Material::Air; 4]; 4];

  for (row, rock_row) in formation.iter().enumerate() {
    for (col, material) in rock_row.iter().enumerate() {
      filled_formation[row][col] = *material
    }
  }

  Ok((input, filled_formation))
}

fn parse_formations(input: &str) -> IResult<&str, Vec<RockFormation>> {
  separated_list1(many_m_n(2, 2, line_ending), parse_formation)(input)
}

fn solve(movements: &Vec<Movement>, rock_formations: &Vec<RockFormation>, amount: usize) -> usize {
  let mut chamber = Chamber::default();
  chamber.solve(&rock_formations, movements, amount);

  chamber.ceiling + chamber.to_add.unwrap_or(0)
}

fn part_1(input: &str) -> usize {
  let movements = parse_movement(input);
  let rock_formations = parse_formations(ROCK_FORMATIONS).unwrap().1;

  solve(&movements, &rock_formations, 2022)
}

fn part_2(input: &str) -> usize {
  let movements = parse_movement(input);
  let rock_formations = parse_formations(ROCK_FORMATIONS).unwrap().1;

  solve(&movements, &rock_formations, 1_000_000_000_000)
}

fn main() {
  println!("{:?}", part_1(INPUT.trim()));
  println!("{:?}", part_2(INPUT.trim()));
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = include_str!("sample");

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT.trim());
    assert_eq!(res, 3068);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT.trim());
    assert_eq!(res, 1514285714288)
  }
}
