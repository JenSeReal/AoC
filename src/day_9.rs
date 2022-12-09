use std::collections::HashSet;

fn parse(input: &str) -> Vec<((i32, i32), usize)> {
  input
    .lines()
    .map(|s| s.split_once(" ").unwrap())
    .map(|(motion, amount)| {
      (
        match motion {
          "R" => (1, 0),
          "L" => (-1, 0),
          "U" => (0, 1),
          "D" => (0, -1),
          _ => unreachable!(),
        },
        amount.parse::<usize>().unwrap(),
      )
    })
    .collect::<Vec<_>>()
}

fn moves<const N: usize>(motions: &Vec<((i32, i32), usize)>) -> usize {
  let mut rope = [(0, 0); N];
  let mut tail = HashSet::from([(0, 0)]);
  for ((x, y), amount) in motions {
    for _ in 0..*amount {
      rope[0].0 += x;
      rope[0].1 += y;

      for i in 1..N {
        let dx = rope[i - 1].0 - rope[i].0;
        let dy = rope[i - 1].1 - rope[i].1;

        if dx.abs() > 1 || dy.abs() > 1 {
          rope[i].0 += dx.signum();
          rope[i].1 += dy.signum();
        }
      }
      tail.insert(rope[N - 1]);
    }
  }

  tail.len()
}

pub(crate) fn part_1(input: &str) -> usize {
  let motions = parse(input);
  moves::<2>(&motions)
}

pub(crate) fn part_2(input: &str) -> usize {
  let motions = parse(input);
  moves::<10>(&motions)
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

  const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 13);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT_2);
    assert_eq!(res, 36)
  }
}
