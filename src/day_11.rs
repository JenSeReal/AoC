use std::{cell::RefCell, collections::VecDeque, fmt::Debug, ops::Range, rc::Rc};

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{self, line_ending, multispace0, newline},
  combinator::{all_consuming, map, opt},
  multi::{many_m_n, separated_list1},
  sequence::{delimited, pair, preceded, separated_pair, terminated},
  Finish, IResult,
};

#[derive(Debug)]
struct KeepAway<O, T>
where
  O: Fn(u64) -> u64,
  T: Fn(u64) -> bool,
{
  players: Vec<Rc<RefCell<Monkey<O, T>>>>,
}

impl<O, T> KeepAway<O, T>
where
  O: Fn(u64) -> u64,
  T: Fn(u64) -> bool,
{
  fn new(players: Vec<Monkey<O, T>>) -> Self {
    KeepAway {
      players: players
        .into_iter()
        .map(|player| Rc::new(RefCell::new(player)))
        .collect(),
    }
  }

  fn play(&mut self, rounds: Range<usize>, common_divisible: Option<u64>) {
    for _ in rounds {
      for monkey in 0..self.players.len() {
        let monkey = self.players[monkey].clone();
        let items = monkey
          .borrow_mut()
          .starting_items
          .drain(..)
          .collect::<Vec<u64>>();
        for item in items {
          monkey.borrow_mut().inspections += 1;
          let new_worry_level = (monkey.borrow().operation)(item);
          let reliefed_worry_level = match common_divisible {
            Some(common_divisible) => new_worry_level % common_divisible,
            None => new_worry_level / 3,
          };

          let receiver = match (monkey.borrow().test)(reliefed_worry_level) {
            true => monkey.borrow().throw_to.0,
            false => monkey.borrow().throw_to.1,
          };

          self
            .players
            .get_mut(receiver)
            .unwrap()
            .borrow_mut()
            .starting_items
            .push(reliefed_worry_level);
        }
      }
    }
  }
}

struct Monkey<O, T>
where
  O: Fn(u64) -> u64,
  T: Fn(u64) -> bool,
{
  starting_items: Vec<u64>,
  operation: O,
  test: T,
  throw_to: (usize, usize),
  inspections: usize,
  divisible: u64,
}

impl<O, T> Debug for Monkey<O, T>
where
  O: Fn(u64) -> u64,
  T: Fn(u64) -> bool,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Monkey")
      .field("starting_items", &self.starting_items)
      .field("throw_to", &self.throw_to)
      .field("inspections", &self.inspections)
      .finish()
  }
}

enum Value {
  Old,
  Num(u64),
}

impl Value {
  fn as_number(&self, old: u64) -> u64 {
    match self {
      Value::Old => old,
      Value::Num(val) => *val,
    }
  }
}

fn parse_value(input: &str) -> IResult<&str, Value> {
  alt((
    map(tag("old"), |_| Value::Old),
    map(complete::u64, Value::Num),
  ))(input)
}

fn parse_function(input: &str) -> IResult<&str, impl Fn(u64) -> u64> {
  let (input, (operand, value)) = preceded(
    tag("new = old "),
    separated_pair(
      alt((complete::char('*'), complete::char('+'))),
      multispace0,
      parse_value,
    ),
  )(input)?;

  Ok((input, move |old| match operand {
    '*' => old * value.as_number(old),
    '+' => old + value.as_number(old),
    _ => unreachable!(),
  }))
}

fn parse_operation(input: &str) -> IResult<&str, impl Fn(u64) -> u64> {
  delimited(
    multispace0,
    preceded(tag("Operation: "), parse_function),
    line_ending,
  )(input)
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<u64>> {
  delimited(
    multispace0,
    preceded(
      tag("Starting items: "),
      separated_list1(tag(", "), complete::u64),
    ),
    line_ending,
  )(input)
}

fn parse_test(input: &str) -> IResult<&str, (u64, impl Fn(u64) -> bool)> {
  let (input, test_value) = delimited(
    multispace0,
    preceded(tag("Test: divisible by "), complete::u64),
    line_ending,
  )(input)?;

  Ok((
    input,
    (test_value, move |worry_level| worry_level % test_value == 0),
  ))
}

fn parse_throw_to(input: &str) -> IResult<&str, (usize, usize)> {
  let (input, to_true) = delimited(
    multispace0,
    preceded(tag("If true: throw to monkey "), complete::u64),
    line_ending,
  )(input)?;
  let (input, to_false) = delimited(
    multispace0,
    preceded(tag("If false: throw to monkey "), complete::u64),
    opt(line_ending),
  )(input)?;

  Ok((input, (to_true as usize, to_false as usize)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey<impl Fn(u64) -> u64, impl Fn(u64) -> bool>> {
  let (input, id) = preceded(
    delimited(tag("Monkey "), complete::u64, tag(":")),
    line_ending,
  )(input)?;
  let (input, starting_items) = parse_starting_items(input)?;
  let (input, operation) = parse_operation(input)?;
  let (input, (test_value, test)) = parse_test(input)?;
  let (input, throw_to) = parse_throw_to(input)?;

  Ok((
    input,
    Monkey {
      starting_items: starting_items.into(),
      operation,
      test,
      throw_to,
      inspections: 0,
      divisible: test_value,
    },
  ))
}

fn parse(input: &str) -> Vec<Monkey<impl Fn(u64) -> u64, impl Fn(u64) -> bool>> {
  input
    .split("\n\n")
    .map(|line| parse_monkey(line).unwrap().1)
    .collect()
}

pub(crate) fn part_1(input: &str) -> usize {
  let monkeys = parse(input);
  let mut game = KeepAway::new(monkeys);

  game.play(0..20, None);

  let mut inspections = game
    .players
    .iter()
    .map(|monkey| monkey.borrow().inspections)
    .collect::<Vec<_>>();

  inspections.sort();

  inspections.iter().rev().take(2).product()
}

pub(crate) fn part_2(input: &str) -> usize {
  let monkeys = parse(input);
  let common_divisible = monkeys
    .iter()
    .map(|monkey| monkey.divisible)
    .product::<u64>();
  let mut game = KeepAway::new(monkeys);

  game.play(0..10_000, Some(common_divisible));

  let mut inspections = game
    .players
    .iter()
    .map(|monkey| monkey.borrow().inspections)
    .collect::<Vec<_>>();

  inspections.sort();

  inspections.iter().rev().take(2).product()
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 10605);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 2713310158)
  }
}
