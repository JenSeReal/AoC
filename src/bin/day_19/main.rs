use std::{convert::Infallible, str::FromStr};

use itertools::Itertools;

const INPUT: &str = include_str!("input");

#[derive(Debug)]
struct State {
  ores: [u32; 4],
  robots: [u32; 4],
  time: u32,
}

impl Default for State {
  fn default() -> Self {
    Self {
      ores: [0; 4],
      robots: [1, 0, 0, 0],
      time: 0,
    }
  }
}

#[derive(Debug)]
enum Material {
  Ore,
  Clay,
  Obsidian,
}

#[derive(Debug)]
struct Blueprint {
  id: usize,
  recipes: [[u32; 4]; 4],
  max_spend: [u32; 4],
}

impl FromStr for Blueprint {
  type Err = Infallible;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut max_spend = [0; 4];
    let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = s
      .split(&[' ', ':'])
      .filter_map(|s| s.parse().ok())
      .collect_tuple()
      .unwrap();

    max_spend[Material::Ore as usize] = max_spend[Material::Ore as usize]
      .max(ore_ore)
      .max(clay_ore)
      .max(obsidian_ore)
      .max(geode_ore);

    max_spend[Material::Clay as usize] = max_spend[Material::Clay as usize].max(obsidian_clay);

    max_spend[Material::Obsidian as usize] =
      max_spend[Material::Obsidian as usize].max(geode_obsidian);

    max_spend[3] = u32::MAX;

    Ok(Blueprint {
      id: id as usize,
      recipes: [
        [ore_ore, 0, 0, 0],
        [clay_ore, 0, 0, 0],
        [obsidian_ore, obsidian_clay, 0, 0],
        [geode_ore, 0, geode_obsidian, 0],
      ],
      max_spend,
    })
  }
}

impl Blueprint {
  fn max_geodes(&self, remaining_minutes: u32) -> u32 {
    let mut max_geodes = 0;
    self.recurse_simulation(State::default(), remaining_minutes, &mut max_geodes);
    max_geodes
  }

  fn recurse_simulation(&self, state: State, max_time: u32, max_geodes: &mut u32) {
    let mut has_recursed = false;
    for i in 0..4 {
      if state.robots[i] == self.max_spend[i] {
        continue;
      }
      let recipe = &self.recipes[i];
      let wait_time = (0..3)
        .filter_map(|ore_type| {
          if recipe[ore_type] == 0 {
            None
          } else if recipe[ore_type] <= state.ores[ore_type] {
            Some(0)
          } else if state.robots[ore_type] == 0 {
            Some(max_time + 1)
          } else {
            Some(
              (recipe[ore_type] - state.ores[ore_type] + state.robots[ore_type] - 1)
                / state.robots[ore_type],
            )
          }
        })
        .max()
        .unwrap();
      let time_finished = state.time + wait_time + 1;
      if time_finished >= max_time {
        continue;
      }
      let mut new_ores = [0; 4];
      let mut new_robots = [0; 4];
      for o in 0..4 {
        new_ores[o] = state.ores[o] + state.robots[o] * (wait_time + 1) - recipe[o];
        new_robots[o] = state.robots[o] + u32::from(o == i);
      }
      let remaining_time = max_time - time_finished;
      if ((remaining_time - 1) * remaining_time) / 2 + new_ores[3] + remaining_time * new_robots[3]
        < *max_geodes
      {
        continue;
      }
      has_recursed = true;
      self.recurse_simulation(
        State {
          ores: new_ores,
          robots: new_robots,
          time: time_finished,
        },
        max_time,
        max_geodes,
      );
    }
    if !has_recursed {
      *max_geodes = std::cmp::max(
        *max_geodes,
        state.ores[3] + state.robots[3] * (max_time - state.time),
      );
    }
  }
}

fn parse(input: &str) -> Vec<Blueprint> {
  input
    .trim()
    .lines()
    .map(FromStr::from_str)
    .filter_map(|s| s.ok())
    .collect()
}

fn part_1(input: &str) -> usize {
  let blueprints = parse(input);

  blueprints
    .iter()
    .map(|bp| bp.max_geodes(24) as usize * bp.id)
    .sum()
}

fn part_2(input: &str) -> usize {
  let blueprints = parse(input);

  blueprints
    .iter()
    .take(3)
    .map(|bp| bp.max_geodes(32) as usize)
    .product::<usize>()
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
    assert_eq!(res, 33);
  }

  #[test]
  fn test_solve_part_2() {
    let res = part_2(TEST_INPUT);
    assert_eq!(res, 3472)
  }
}
