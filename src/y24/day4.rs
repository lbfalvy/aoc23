use itertools::Itertools;

use crate::common::fetch_input_lines;

fn table_get(data: &[Vec<char>], i: i64, j: i64) -> Option<char> {
  let i: usize = i.try_into().ok()?;
  let j: usize = j.try_into().ok()?;
  data.get(i)?.get(j).copied()
}

pub fn day4() {
  let data =
    fetch_input_lines(4).map(|l| l.chars().collect_vec()).collect_vec();
  let mut xmas_count = 0;
  let mut x_mas_count = 0;
  for (i, j) in
    (0..data.len() as i64).cartesian_product(0..data[0].len() as i64)
  {
    for (di, dj) in (-1..=1).cartesian_product(-1..=1) {
      'xmas: {
        for (offset, char) in "XMAS".chars().enumerate() {
          let char_i = i + di * offset as i64;
          let char_j = j + dj * offset as i64;
          if table_get(&data, char_i, char_j) != Some(char) {
            break 'xmas;
          }
        }
        xmas_count += 1;
      }
    }
    'x_mas: {
      if table_get(&data, i, j) != Some('A') {
        break 'x_mas;
      }
      for slope in [1, -1] {
        let coords = [1, -1].map(|side| (i + side, j + side * slope));
        let Some(opp_corners) = (coords.into_iter())
          .map(|(i, j)| table_get(&data, i, j))
          .collect::<Option<Vec<char>>>()
        else {
          break 'x_mas;
        };
        if !matches!(&opp_corners[..], ['M', 'S'] | ['S', 'M']) {
          break 'x_mas;
        }
      }
      x_mas_count += 1;
    }
  }
  eprintln!("XMAS count: {xmas_count}, X-MAS count: {x_mas_count}")
}
