use itertools::Itertools;

use crate::common::fetch_input_lines;

pub fn day1() {
  let data = fetch_input_lines(1)
    .map(|s| s.split("   ").map(|s| s.trim().parse::<u32>().unwrap()).collect_tuple::<(u32, u32)>().unwrap())
    .collect_vec();
  let (mut left, mut right) = data.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
  left.sort();
  right.sort();
  let answer_a = left.iter().zip(&right).map(|(a, b)| a.abs_diff(*b)).sum::<u32>();
  let answer_b = left.iter().map(|n| n * (right.iter().filter(|x| *x == n).count() as u32)).sum::<u32>();
  eprintln!("A: {answer_a}, B: {answer_b}")
}
