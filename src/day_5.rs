use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{self, alpha1, char, line_ending, newline, not_line_ending},
  multi::{many1, many_till, separated_list1},
  sequence::delimited,
  IResult,
};

#[derive(Debug, Clone)]
struct Container {
  id: String,
}

fn parse_container(input: &str) -> IResult<&str, Option<Container>> {
  let (input, container) = alt((tag("   "), delimited(char('['), alpha1, char(']'))))(input)?;
  let container = match container {
    "   " => None,
    val => Some(Container {
      id: val.to_string(),
    }),
  };
  Ok((input, container))
}

fn parse_containers(input: &str) -> IResult<&str, Vec<Option<Container>>> {
  let (input, containers) = separated_list1(tag(" "), parse_container)(input)?;
  Ok((input, containers))
}

#[derive(Debug)]

struct Move {
  quantity: u32,
  from: u32,
  to: u32,
}

fn parse_move(input: &str) -> IResult<&str, Move> {
  let (input, (_, quantity)) = many_till(tag("move "), complete::u32)(input)?;
  let (input, (_, from)) = many_till(tag(" from "), complete::u32)(input)?;
  let (input, (_, to)) = many_till(tag(" to "), complete::u32)(input)?;

  Ok((input, Move { quantity, from, to }))
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<Container>>, Vec<Move>)> {
  let (input, container_rows) = separated_list1(newline, parse_containers)(input)?;
  let (input, _) = line_ending(input)?;
  let (input, _) = not_line_ending(input)?;
  let (input, _) = many1(line_ending)(input)?;
  let (_, moves) = separated_list1(line_ending, parse_move)(input)?;

  let container_stacks = container_rows.iter().rev().fold(
    vec![vec![]; container_rows[0].len()],
    |mut acc: Vec<Vec<Container>>, container_row| {
      for (idx, container) in container_row.iter().enumerate() {
        if container.is_some() {
          acc[idx].push(container.clone().unwrap())
        }
      }
      acc
    },
  );

  Ok(("", (container_stacks, moves)))
}

pub(crate) fn part_1(input: &str) -> String {
  let (_, (mut containers, moves)) = parse(input).unwrap();
  moves.iter().for_each(|mv| {
    let len = containers[mv.from as usize - 1].len();
    let containers_to_move = containers[mv.from as usize - 1]
      .drain((len - mv.quantity as usize)..)
      .rev()
      .collect::<Vec<_>>();

    containers[mv.to as usize - 1].extend(containers_to_move);
  });

  let ids = containers.iter().fold("".to_string(), |mut acc, stack| {
    match stack.last() {
      Some(c) => acc.push_str(&c.id),
      None => (),
    };
    acc
  });

  ids
}

pub(crate) fn part_2(input: &str) -> String {
  let (_, (mut containers, moves)) = parse(input).unwrap();
  moves.iter().for_each(|mv| {
    let len = containers[mv.from as usize - 1].len();
    let containers_to_move = containers[mv.from as usize - 1]
      .drain((len - mv.quantity as usize)..)
      .collect::<Vec<_>>();

    containers[mv.to as usize - 1].extend(containers_to_move);
  });

  let ids = containers.iter().fold("".to_string(), |mut acc, stack| {
    match stack.last() {
      Some(c) => acc.push_str(&c.id),
      None => (),
    };
    acc
  });

  ids
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, "CMZ");
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, "MCD")
  }
}
