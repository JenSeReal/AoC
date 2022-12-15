use std::fs::read_to_string;

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{self, line_ending},
  combinator::map,
  multi::separated_list1,
  sequence::preceded,
  IResult,
};

#[derive(Debug)]
enum Instruction {
  Addx(i32),
  Noop,
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
  map(preceded(tag("addx "), complete::i32), Instruction::Addx)(input)
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
  map(tag("noop"), |_| Instruction::Noop)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
  let (input, instruction) = alt((parse_addx, parse_noop))(input)?;
  Ok((input, instruction))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
  let (input, instructions) = separated_list1(line_ending, parse_instruction)(input)?;

  Ok((input, instructions))
}

pub(crate) fn part_1(input: &str) -> i32 {
  let (_, instructions) = parse(input).unwrap();

  let interesting_signal_strengths = [20, 60, 100, 140, 180, 220];
  let mut register = 1;
  let mut cycle = 1;

  instructions
    .iter()
    .filter_map(|instruction| {
      let mut signal = None;
      cycle += 1;

      if interesting_signal_strengths.contains(&cycle) {
        signal = Some(cycle * register)
      }

      match instruction {
        Instruction::Addx(amount) => {
          register += amount;
          cycle += 1;
          if interesting_signal_strengths.contains(&cycle) {
            signal = Some(cycle * register)
          }
        }
        Instruction::Noop => (),
      };
      signal
    })
    .sum()
}

const CRT_COLUMNS: usize = 40;
const CRT_ROWS: usize = 6;
const CRT_SIZE: usize = CRT_COLUMNS * CRT_ROWS;
const SPRITE_WIDTH: u32 = 3;

#[derive(Debug, Clone, Copy)]
enum Pixel {
  Light,
  Dark,
}

impl Pixel {
  fn as_char(&self) -> char {
    match self {
      Pixel::Light => '.',
      Pixel::Dark => '#',
    }
  }
}

#[derive(Debug)]
struct Crt<const N: usize> {
  x: i32,
  cycle: usize,
  screen: [Pixel; N],
}

impl<const N: usize> Crt<N> {
  fn new() -> Self {
    Self {
      x: 1,
      cycle: 1,
      screen: [Pixel::Light; N],
    }
  }

  fn calculate_pixel(&mut self) {
    let current_column = (self.cycle - 1) % CRT_COLUMNS;
    self.screen[self.cycle - 1] = match (current_column as i32).abs_diff(self.x) <= SPRITE_WIDTH / 2
    {
      true => Pixel::Dark,
      false => Pixel::Light,
    }
  }

  fn draw(&mut self, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
      self.calculate_pixel();
      self.cycle += 1;

      match instruction {
        Instruction::Addx(amount) => {
          self.calculate_pixel();
          self.cycle += 1;
          self.x += amount
        }
        Instruction::Noop => (),
      }
    }

    self
      .screen
      .chunks(CRT_COLUMNS)
      .map(|row| row.iter().map(|px| px.as_char()).collect::<String>())
      .collect::<Vec<_>>()
      .join("\n")
  }
}

pub(crate) fn part_2(input: &str) -> String {
  let (_, instructions) = parse(input).unwrap();
  let mut screen = Crt::<CRT_SIZE>::new();
  screen.draw(&instructions)
}

fn main() {
  let input = read_to_string("assets/day_10").unwrap();
  let part_1 = part_1(&input);
  println!("Part 1: {}", part_1);
  let part_2 = part_2(&input);
  println!("Part 2: {}", part_2);
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 13140);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(
      res,
      "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    )
  }
}
