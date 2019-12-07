use std::collections::HashMap;

use crate::util;

pub fn run() {
  let ids = util::read_file_by_lines(2018, 2);

  println!("Checksum is {}.", checksum(&ids));
  println!("Common letter between similar IDs are {}.", find_boxes(&ids));
}

fn checksum(ids: &[String]) -> u32 {
  let mut twos = 0;
  let mut threes = 0;
  for id in ids {
    let mut counts: HashMap<char, u8> = HashMap::new();

    for c in id.chars() {
      let count = *counts.get(&c).unwrap_or(&0);

      counts.insert(c, count + 1);
    }

    if counts.values().any(|c| *c == 2) {
      twos += 1;
    }
    if counts.values().any(|c| *c == 3) {
      threes += 1;
    }
  }

  twos * threes
}

fn find_boxes(ids: &[String]) -> String {
  for id1 in ids {
    'inner: for id2 in ids {
      let mut differences = 0;
      if id1 == id2 {
        continue;
      }
      for (i, c) in id1.char_indices() {
        if c != id2.chars().nth(i).unwrap() {
          differences += 1;
        }

        if differences == 2 {
          continue 'inner;
        }
      }

      return get_id((id1, id2));
    }
  }

  String::new()
}

fn get_id(boxes: (&str, &str)) -> String {
  let (box1, box2) = boxes;
  box1.char_indices().fold(String::new(), |acc, (i, c)| {
    if c == box2.chars().nth(i).unwrap() {
      format!("{}{}", acc, c)
    } else {
      acc
    }
  })
}
