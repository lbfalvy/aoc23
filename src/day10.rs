use itertools::Itertools;

use crate::common::fetch_input_lines;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}
impl Direction {
  pub const ALL: [Direction; 4] =
    [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
  pub fn opposite(self) -> Direction {
    use Direction::*;
    match self {
      Down => Up,
      Up => Down,
      Left => Right,
      Right => Left,
    }
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(usize, usize);
impl Point {
  pub fn mov(self, direction: Direction) -> Self {
    match direction {
      Direction::Up => Self(self.0 - 1, self.1),
      Direction::Right => Self(self.0, self.1 + 1),
      Direction::Down => Self(self.0 + 1, self.1),
      Direction::Left => Self(self.0, self.1 - 1),
    }
  }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pipe(char);
impl Pipe {
  const STATES: &'static str = "|-LF7J";
  pub fn new(state: char) -> Self {
    assert!(Self::STATES.contains(state), "Invalid pipe");
    Self(state)
  }

  pub fn ends(self) -> [Direction; 2] {
    match self.0 {
      '|' => [Direction::Up, Direction::Down],
      '-' => [Direction::Left, Direction::Right],
      'L' => [Direction::Up, Direction::Right],
      'F' => [Direction::Right, Direction::Down],
      '7' => [Direction::Down, Direction::Left],
      'J' => [Direction::Left, Direction::Up],
      _ => panic!("Invalid character"),
    }
  }

  pub fn from_ends(mut ends: [Direction; 2]) -> Self {
    ends.sort_unstable();
    match ends {
      [Direction::Up, Direction::Down] => Self::new('|'),
      [Direction::Right, Direction::Left] => Self::new('-'),
      [Direction::Up, Direction::Right] => Self::new('L'),
      [Direction::Right, Direction::Down] => Self::new('F'),
      [Direction::Down, Direction::Left] => Self::new('7'),
      [Direction::Up, Direction::Left] => Self::new('J'),
      _ => panic!("Invalid pair: {ends:?}"),
    }
  }

  pub fn is_open(self, direction: Direction) -> bool {
    self.ends().contains(&direction)
  }

  pub fn other_end(self, direction: Direction) -> Direction {
    self.ends().into_iter().filter(|d| d != &direction).exactly_one().unwrap()
  }
}

fn lookup<T>(table: &[Vec<T>], point: Point) -> &T { &table[point.0][point.1] }

fn lookup_mut<T>(table: &mut [Vec<T>], point: Point) -> &mut T {
  &mut table[point.0][point.1]
}

pub fn day10() {
  let mut starting_pos = Point(0, 0);
  let mut table = fetch_input_lines(10)
    .enumerate()
    .map(|(col, l)| {
      l.chars()
        .enumerate()
        .map(|(line, state)| match state {
          'S' => {
            starting_pos = Point(col, line);
            None // To be replaced
          },
          '.' => None,
          state => Some(Pipe::new(state)),
        })
        .collect_vec()
    })
    .collect_vec();

  let ends = (Direction::ALL.iter().copied())
    .filter(|d| {
      lookup(&table, starting_pos.mov(*d))
        .is_some_and(|p| p.is_open(d.opposite()))
    })
    .collect_vec();
  *lookup_mut(&mut table, starting_pos) =
    Some(Pipe::from_ends(ends.clone().try_into().unwrap()));
  // Walk starts from 2nd tile both ways, Half walk length starts from 1.
  // Half walk length must be at least 2 but may be odd.
  let mut d = ends[0];
  let mut pos = starting_pos;
  let mut steps = 0;
  let mut area = 0i64;
  loop {
    steps += 1;
    match d {
      Direction::Left => area -= pos.0 as i64,
      Direction::Right => area += pos.0 as i64,
      _ => (),
    }
    pos = pos.mov(d);
    let pipe = lookup(&table, pos).unwrap();
    // println!("Moved towards {d:?} into {pipe:?} at {pos:?}, area is {area}");
    d = pipe.other_end(d.opposite());
    if pos == starting_pos {
      break;
    }
  }
  println!(
    "length = {steps}; /2 = {}; area = {area}, inner = {}",
    steps / 2,
    area - steps / 2 + 1
  )
}
