use itertools::Itertools;

use crate::common::fetch_input;

pub fn parse_digits(s: &str) -> Option<(u64, &str)> {
  let mut chars = s.chars();
  let digits = chars
    .by_ref()
    .peeking_take_while(|c| c.is_ascii_digit())
    .take(3)
    .collect::<String>();
  (!digits.is_empty()).then(|| (digits.parse().unwrap(), chars.as_str()))
}

pub fn day3() {
  let text = fetch_input(3);
  let mut enabled = true;
  let mut sum = 0;
  let mut tail: &str = &text;
  while !tail.is_empty() {
    tail = if let Some(sub_tail) = tail.strip_prefix("do()") {
      enabled = true;
      sub_tail
    } else if let Some(sub_tail) = tail.strip_prefix("don't()") {
      enabled = false;
      sub_tail
    } else {
      'b: {
        let Some(tail_fst_num) = tail.strip_prefix("mul(") else {
          break 'b &tail[1..];
        };
        let Some((a, tail_comma)) = parse_digits(tail_fst_num) else {
          break 'b tail_fst_num;
        };
        let Some(tail_snd_num) = tail_comma.strip_prefix(',') else {
          break 'b tail_comma;
        };
        let Some((b, tail_cparen)) = parse_digits(tail_snd_num) else {
          break 'b tail_snd_num;
        };
        let Some(new_tail) = tail_cparen.strip_prefix(')') else {
          break 'b tail_cparen;
        };
        if enabled {
          sum += a * b
        }
        new_tail
      }
    }
  }
  eprintln!("Sum of mults: {sum}")
}
