use crate::common::{fetch_input_lines};

struct Rgb {
  r: usize,
  g: usize,
  b: usize,
}
impl Rgb {
  fn subset(&self, other: &Rgb) -> bool {
    self.r <= other.r && self.g <= other.g && self.b <= other.b
  }
  fn union(&self, other: &Rgb) -> Rgb {
    Rgb {
      b: self.b.max(other.b),
      r: self.r.max(other.r),
      g: self.g.max(other.g),
    }
  }
}

struct Game {
  draws: Vec<Rgb>,
  id: usize,
}

fn parse_line(line: &str) -> Game {
  let (head, draws) = line.split_once(':').unwrap();
  let id: usize = head.strip_prefix("Game ").unwrap().parse().unwrap();
  let draws = draws
    .split(';')
    .map(|draw_str| {
      let mut draw = Rgb { b: 0, g: 0, r: 0 };
      for component in draw_str.split(',') {
        let (num, col) = component.trim().split_once(' ').unwrap();
        let num: usize = num.trim().parse().unwrap();
        match col.trim() {
          "red" => draw.r = num,
          "green" => draw.g = num,
          "blue" => draw.b = num,
          _ => panic!("unrecognized colour"),
        }
      }
      draw
    })
    .collect();
  Game { id, draws }
}

pub fn day2(part_two: bool) {
  let constraint = Rgb { r: 12, g: 13, b: 14 };
  let games = fetch_input_lines(2).map(|s| parse_line(&s));
  if !part_two {
    let output = games
      .filter(|game| game.draws.iter().all(|draw| draw.subset(&constraint)))
      .map(|game| game.id)
      .sum::<usize>();
    println!("day2 result: {output}")
  } else {
    let output = games
      .map(|game| game.draws.iter().fold(Rgb{ r: 0, g: 0, b: 0 }, |acc, draw| acc.union(draw)))
      .map(|minset| minset.r * minset.g * minset.b)
      .sum::<usize>();
    println!("day2 part 2 result: {output}")
  }
}
