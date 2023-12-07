
use itertools::Itertools;

use crate::common::fetch_input;

const TEXT_DIGITS: &[&str] =
  &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn day1(part2: bool) {
  // map strings to values
  let mut str_digit_tbl = ((1..=9).map(|d| (d.to_string(), d))).collect::<Vec<_>>();
  if part2 {
    str_digit_tbl.extend(TEXT_DIGITS.iter().enumerate().map(|(i, s)| (s.to_string(), i + 1)))
  }
  let output: usize = fetch_input(1)
    .split('\n')
    .filter_map(|case| {
      if case.is_empty() {
        return None;
      }
      // values with positions where they're found
      let mut positions = (str_digit_tbl.iter())
        .flat_map(|(k, v)| case.match_indices(k).map(|(i, _)| (*v, i)))
        .collect::<Vec<_>>();
      positions.sort_unstable_by_key(|(_, pos)| *pos);
      let first = positions.first().unwrap().0;
      let last = positions.last().unwrap().0;
      println!(
        "{case} => {first}{last} - {}",
        positions.iter().map(|(val, pos)| format!("{val} at {pos}")).join(", ")
      );
      Some(first * 10 + last)
    })
    .sum();
  println!("day1 result: {output}")
}
