use hashbrown::HashMap;

use crate::common::fetch_input_lines;

pub fn read_leading_number(
  text: impl IntoIterator<Item = char>,
) -> (usize, usize) {
  (text.into_iter())
    .map_while(|c| c.to_digit(10))
    .fold((0, 0), |(i, acc), digit| (i + 1, acc * 10 + digit as usize))
}

pub fn day3() {
  let board = fetch_input_lines(3)
    .map(|s| s.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let mut part_numbers = HashMap::<(usize, usize), usize>::new();
  let mut star_neighbours = HashMap::<(usize, usize), Vec<usize>>::new();
  for row in 0..board.len() {
    let mut was_prev_num = false;
    for col in 0..board[0].len() {
      let (digits, number) =
        read_leading_number(board[row][col..].iter().cloned());
      if !was_prev_num && 0 < digits {
        let rmin = row.saturating_sub(1);
        let cmin = col.saturating_sub(1);
        let rmax = board.len().min(row + 2) - 1;
        let cmax = board[0].len().min(col + digits + 1) - 1;
        println!(
          "Found {number} at ({row},{col}), search area from ({rmin},{cmin}) to ({rmax},{cmax})"
        );
        // search area defined by board edges
        for (sr, sc) in
          (rmin..=rmax).flat_map(|r| (cmin..=cmax).map(move |c| (r, c)))
        {
          let symbol = board[sr][sc];
          if symbol != '.' && !symbol.is_ascii_digit() {
            part_numbers.insert((row, col), number);
            println!("Part number because of {symbol} at ({sr},{sc})",);
          }
          if symbol == '*' {
            let (_, numbers) = (star_neighbours.raw_entry_mut())
              .from_key(&(sr, sc))
              .and_modify(|_, v| v.push(number))
              .or_insert((sr, sc), vec![number]);
            match &numbers[..] {
              [first] => println!("Gear candidate at ({row},{col}): [{first}]"),
              [first, second] => println!(
                "Gear at ({row},{col}): {first}*{second} = {}",
                first * second
              ),
              _ => panic!("Invalid gear with {} neighbours", numbers.len()),
            }
          }
        }
      }
      was_prev_num = 0 < digits;
    }
  }
  println!(
    "The sum of the part numbers is {}",
    part_numbers.values().sum::<usize>()
  );
  println!(
    "The sum of the gear ratios is {}",
    star_neighbours
      .values()
      .filter(|g| g.len() == 2)
      .map(|v| v.iter().product::<usize>())
      .sum::<usize>()
  )
}
