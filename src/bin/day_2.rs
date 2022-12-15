use std::{cmp::Ordering, convert::Infallible, fs::read_to_string, str::FromStr};

#[derive(Debug)]
enum Outcome {
  Lose = 1,
  Draw,
  Win,
}

impl FromStr for Outcome {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "X" => Ok(Self::Lose),
      "Y" => Ok(Self::Draw),
      "Z" => Ok(Self::Win),
      _ => panic!(),
    }
  }
}

#[derive(Debug, PartialEq, Eq, Ord)]
enum Hand {
  Rock = 1,
  Paper,
  Scissors,
}

impl Hand {
  fn result(&self, other: &Self) -> u32 {
    match self.partial_cmp(other) {
      Some(Ordering::Equal) => 3,
      Some(Ordering::Greater) => 6,
      Some(Ordering::Less) => 0,
      None => panic!(),
    }
  }

  fn move_decider(&self, outcome: Outcome) -> Self {
    match (self, outcome) {
      (Hand::Rock, Outcome::Lose) => Self::Scissors,
      (Hand::Rock, Outcome::Win) => Self::Paper,
      (Hand::Paper, Outcome::Lose) => Self::Rock,
      (Hand::Paper, Outcome::Win) => Self::Scissors,
      (Hand::Scissors, Outcome::Lose) => Self::Paper,
      (Hand::Scissors, Outcome::Win) => Self::Rock,
      (Hand::Rock, Outcome::Draw) => Self::Rock,
      (Hand::Paper, Outcome::Draw) => Self::Paper,
      (Hand::Scissors, Outcome::Draw) => Self::Scissors,
    }
  }
}

impl FromStr for Hand {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "A" | "X" => Ok(Self::Rock),
      "B" | "Y" => Ok(Self::Paper),
      "C" | "Z" => Ok(Self::Scissors),
      _ => panic!(),
    }
  }
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (Self::Rock, Self::Scissors) => Some(Ordering::Greater),
      (Self::Scissors, Self::Rock) => Some(Ordering::Less),
      _ => Some(self.cmp(other)),
    }
  }
}

fn parse_part_1(input: &str) -> impl Iterator<Item = (Hand, Hand)> + '_ {
  input.lines().map(|round| {
    round
      .split_once(" ")
      .map(|(player_one, player_two)| {
        (
          player_one.parse::<Hand>().unwrap(),
          player_two.parse::<Hand>().unwrap(),
        )
      })
      .unwrap()
  })
}

fn parse_part_2(input: &str) -> impl Iterator<Item = (Hand, Outcome)> + '_ {
  input.lines().map(|round| {
    round
      .split_once(" ")
      .map(|(hand, outcome)| {
        (
          hand.parse::<Hand>().unwrap(),
          outcome.parse::<Outcome>().unwrap(),
        )
      })
      .unwrap()
  })
}

pub(crate) fn part_1(input: &str) -> u32 {
  parse_part_1(input)
    .map(|(p1, p2)| p2.result(&p1) + p2 as u32)
    .sum()
}

pub(crate) fn part_2(input: &str) -> u32 {
  parse_part_2(input)
    .map(|(player_one, outcome)| {
      let player_two = player_one.move_decider(outcome);
      player_two.result(&player_one) + player_two as u32
    })
    .sum()
}

fn main() {
  let input = read_to_string("assets/day_2").unwrap();
  let part_1 = part_1(&input);
  println!("Part 1: {}", part_1);
  let part_2 = part_2(&input);
  println!("Part 2: {}", part_2);
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "A Y
B X
C Z";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 15);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 12)
  }
}
