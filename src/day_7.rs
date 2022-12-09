use std::{collections::HashMap, path::PathBuf};

use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  combinator::{all_consuming, map},
  sequence::{preceded, separated_pair},
  Finish, IResult,
};

const THRESHOLD: usize = 100000;

#[derive(Debug)]
enum Entry {
  Dir(PathBuf),
  File(u64, PathBuf),
}

#[derive(Debug)]
enum Command {
  Cd(PathBuf),
  Ls,
}

#[derive(Debug)]
enum Operation {
  Command(Command),
  Entry(Entry),
}

fn parse_path(input: &str) -> IResult<&str, PathBuf> {
  map(
    take_while1(|c| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
    Into::into,
  )(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
  map(preceded(tag("cd "), parse_path), Command::Cd)(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
  map(tag("ls"), |_| Command::Ls)(input)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
  let parse_file = map(
    separated_pair(nom::character::complete::u64, tag(" "), parse_path),
    |(size, path)| Entry::File(size, path),
  );
  let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);
  alt((parse_file, parse_dir))(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
  let (input, _) = tag("$ ")(input)?;
  alt((parse_ls, parse_cd))(input)
}

fn parse_line(input: &str) -> IResult<&str, Operation> {
  let (input, operation) = alt((
    map(parse_command, Operation::Command),
    map(parse_entry, Operation::Entry),
  ))(input)?;
  Ok((input, operation))
}

fn operations_to_directories(ops: &Vec<Operation>) -> HashMap<PathBuf, u64> {
  let mut history = vec![];
  ops.iter().fold(HashMap::new(), |mut acc, operation| {
    match operation {
      Operation::Command(cmd) => match cmd {
        Command::Cd(path) => match path.to_str().unwrap() {
          ".." => {
            history.pop();
          }
          "/" => {
            history.clear();
            history.push("/")
          }
          path => history.push(path),
        },
        Command::Ls => (),
      },
      Operation::Entry(entry) => match entry {
        Entry::Dir(_) => (),
        Entry::File(size, _) => {
          for (idx, _) in history.iter().enumerate() {
            let path = PathBuf::from_iter(&history[..=idx]);
            *acc.entry(path).or_insert(0) += size
          }
        }
      },
    }
    acc
  })
}

pub(crate) fn part_1(input: &str) -> u64 {
  let parsed = input
    .lines()
    .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
    .collect::<Vec<_>>();

  let dirs = operations_to_directories(&parsed);

  dirs.into_values().filter(|val| *val <= 100_000).sum()
}

pub(crate) fn part_2(input: &str) -> u64 {
  let parsed = input
    .lines()
    .map(|line| all_consuming(parse_line)(line).finish().unwrap().1)
    .collect::<Vec<_>>();

  let dirs = operations_to_directories(&parsed);

  let total = 70_000_000;
  let needed = 30_000_000;
  let used = dirs.get(&PathBuf::from("/")).unwrap();
  let unused = total - used;

  dirs
    .into_values()
    .filter(|size| unused + size >= needed)
    .min()
    .unwrap()
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 95437);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 24933642)
  }
}
