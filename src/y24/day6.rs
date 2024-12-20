use itertools::Itertools;

use crate::common::{fetch_input, fetch_input_lines};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Heading {
  Up,
  Left,
  Down,
  Right
}
impl Heading {
  pub fn next(self) -> Self {
    match self {
      Self::Down => Self::Left,
      Self::Left => Self::Up,
      Self::Up => Self::Right,
      Self::Right => Self::Down
    }
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Tile {
  None,
  Wall,
  Trail(Heading),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct V2i(i64, i64);
impl V2i {
  fn next(self, hdg: Heading) -> Self {
    match hdg {
      Heading::Right => Self(self.0 + 1, self.1),
      Heading::Left => Self(self.0 - 1, self.1),
      Heading::Up => Self(self.0, self.1 - 1),
      Heading::Down => Self(self.0, self.1 + 1),
    }
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct State(Heading, V2i);
impl State {
  fn next(self, board: &Board) -> Self {
    if board.is_bounce(self.1.next(self.0)) {
      Self(self.0.next(), self.1)
    } else {
      Self(self.0, self.1.next(self.0))
    }
  }
}

#[derive(Clone)]
struct Board(Vec<Vec<Tile>>);
impl Board {
  fn in_bounds(&self, pos: V2i) -> bool {
    (0 <= pos.0 && 0 <= pos.1)
      && pos.0 < self.0[0].len().try_into().unwrap()
      && pos.1 < self.0.len().try_into().unwrap()
  }
  fn get(&self, pos: V2i) -> Option<Tile> {
    self.in_bounds(pos).then(|| self.0[pos.1 as usize][pos.0 as usize])
  }
  fn set(&mut self, pos: V2i, tile: Tile) {
    assert!(self.in_bounds(pos), "{pos:?} out of bounds");
    self.0[pos.1 as usize][pos.0 as usize] = tile;
  }
  fn is_bounce(&self, pos: V2i) -> bool {
    self.get(pos) == Some(Tile::Wall)
  }
}

pub fn day6() {
  let mut data = fetch_input_lines(6)
    .map(|l| l.chars().collect_vec())
    .collect_vec();
  let (mut hdg, mut x, mut y) = (data.iter().enumerate())
    .find_map(|(y, l)| 
      (l.iter().enumerate())
      .find_map(|(x, c)| HEADINGS.contains(c).then_some((*c, x as i32, y as i32))))
    .expect("Guard not found!");
  eprintln!("Starting from {x};{y}, limits are {};{}", data[0].len(), data.len());
  let mut bounces = 0;
  loop {
    let (x2, y2) = forward(hdg, x, y);
    if 0 > x2 || 0 > y2 || y2 >= data.len() as i32 || x2 >= data[0].len() as i32 {
      data[y as usize][x as usize] = hdg;
      break;
    }
    if data[y as usize][x as usize] == clockwise(hdg) &&
      data[y2 as usize][x2 as usize] == '.' {
        bounces += 1;
    }
    data[y as usize][x as usize] = hdg;
    if data[y2 as usize][x2 as usize] == '#' {
      eprintln!("Bounced at {x};{y}{hdg}");
      hdg = clockwise(hdg);
      continue;
    }
    x = x2;
    y = y2;
  }
  let all_visited: usize = data.iter().map(|l| l.iter().filter(|c| HEADINGS.contains(&c)).count()).sum();
  eprintln!("Total visited: {all_visited}, potential loop bounces: {bounces}")
}