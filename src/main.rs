#![allow(dead_code)]
#![allow(unused)]

use std::fs::read_to_string;

mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
  // let input_day_1 = read_to_string("assets/day_1").unwrap();
  // let res = day_1::part_1(&input_day_1);
  // let res = day_1::part_2(&input_day_1);

  // let input_day_2 = read_to_string("assets/day_2").unwrap();
  // let res = day_2::part_1(&input_day2);
  // let res = day_2::part_2(&input_day_2);

  // let input_day_3 = read_to_string("assets/day_3").unwrap();
  // let res = day_3::part_1(&input_day_3);
  // let res = day_3::part_2(&input_day_3);

  // let input_day_4 = read_to_string("assets/day_4").unwrap();
  // let res = day_4::part_1(&input_day_4);
  // let res = day_4::part_2(&input_day_4);

  // let input_day_5 = read_to_string("assets/day_5").unwrap();
  // let res = day_5::part_1(&input_day_5);
  // let res = day_5::part_2(&input_day_5);

  // let input_day_6 = read_to_string("assets/day_6").unwrap();
  // let res = day_6::part_1(&input_day_6);
  // let res = day_6::part_2(&input_day_6);

  // let input_day_7 = read_to_string("assets/day_7").unwrap();
  // let res = day_7::part_1(&input_day_7);
  // let res = day_7::part_2(&input_day_7);

  // let input_day_8 = read_to_string("assets/day_8").unwrap();
  // let res = day_8::part_1(&input_day_8);
  // let res = day_8::part_2(&input_day_8);

  // let input_day_9 = read_to_string("assets/day_9").unwrap();
  // let res = day_9::part_1(&input_day_9);
  // let res = day_9::part_2(&input_day_9);

  let input_day_10 = read_to_string("assets/day_10").unwrap();
  let res = day_10::part_1(&input_day_10);
  let res = day_10::part_2(&input_day_10);

  println!("{}", res);
}
