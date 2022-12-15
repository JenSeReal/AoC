use std::fs::read_to_string;

fn parse(input: &str) -> Vec<Vec<u32>> {
  let y = input.lines().count();

  input
    .lines()
    .rev()
    .enumerate()
    .fold(vec![vec![]; y], |mut forest, (y, line)| {
      let tree_line = line
        .chars()
        .map(|height| height.to_digit(10).unwrap())
        .collect::<Vec<_>>();
      forest[y] = tree_line;
      forest
    })
}

fn is_visible(height: &u32, (x, y): (usize, usize), trees: &Vec<Vec<u32>>) -> bool {
  if x == 0 || y == 0 || x == trees.len() - 1 || y == trees[x].len() - 1 {
    return true;
  };

  let visible_left = (0..x).rev().all(|position| trees[y][position] < *height);
  let visible_right = (x + 1..trees[y].len()).all(|position| trees[y][position] < *height);
  let visible_top = (y + 1..trees[x].len()).all(|position| trees[position][x] < *height);
  let visible_bottom = (0..y).rev().all(|position| trees[position][x] < *height);

  visible_left || visible_right || visible_top || visible_bottom
}

fn scenic_score(height: &u32, (x, y): (usize, usize), trees: &Vec<Vec<u32>>) -> usize {
  let mut score_left = 0;
  for position in (0..x).rev() {
    score_left += 1;
    if trees[y][position] >= *height {
      break;
    }
  }

  let mut score_right = 0;
  for position in x + 1..trees[y].len() {
    score_right += 1;
    if trees[y][position] >= *height {
      break;
    }
  }

  let mut score_top = 0;
  for position in y + 1..trees[x].len() {
    score_top += 1;
    if trees[position][x] >= *height {
      break;
    }
  }

  let mut score_bottom = 0;
  for position in (0..y).rev() {
    score_bottom += 1;
    if trees[position][x] >= *height {
      break;
    }
  }
  score_left * score_right * score_top * score_bottom
}

pub(crate) fn part_1(input: &str) -> usize {
  let forest = parse(input);

  forest
    .iter()
    .enumerate()
    .map(|(y, tree_line)| {
      tree_line
        .iter()
        .enumerate()
        .filter(|(x, height)| is_visible(height, (*x, y), &forest))
        .count()
    })
    .sum()
}

pub(crate) fn part_2(input: &str) -> usize {
  let forest = parse(input);

  forest
    .iter()
    .enumerate()
    .map(|(y, tree_line)| {
      tree_line
        .iter()
        .enumerate()
        .map(|(x, height)| scenic_score(height, (x, y), &forest))
        .max()
        .unwrap()
    })
    .max()
    .unwrap()
}

fn main() {
  let input = read_to_string("assets/day_8").unwrap();
  let part_1 = part_1(&input);
  println!("Part 1: {}", part_1);
  let part_2 = part_2(&input);
  println!("Part 2: {}", part_2);
}

#[cfg(test)]
pub(crate) mod tests {
  use super::*;

  const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

  #[test]
  fn test_solve_part_1() {
    let res = part_1(TEST_INPUT);
    assert_eq!(res, 21);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 8)
  }
}
