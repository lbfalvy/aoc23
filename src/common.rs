use std::fs::File;
use std::io::Read;

pub fn fetch_input(day: usize) -> String {
  let mut input = String::new();
  File::open(format!("./inputs/day{day}.txt"))
    .unwrap()
    .read_to_string(&mut input)
    .unwrap();
  input
}

pub fn fetch_input_lines(day: usize) -> impl Iterator<Item = String> {
  let lines = fetch_input(day)
    .split('\n')
    .filter(|s| !s.is_empty())
    .map(|s| s.to_owned())
    .collect::<Vec<_>>();
  lines.into_iter()
}
