use crate::util;
use std::{
  cmp::Ordering,
  collections::{HashMap, HashSet},
};

pub fn run() {
  let file = util::read_file_by_lines(2019, 3);
  let paths: Vec<Vec<String>> = file
    .iter()
    .map(|line| line.split(',').map(|x| x.to_string()).collect())
    .collect();

  let intersections = get_intersections(paths);
  println!("Closest intersection: {}", get_closest(intersections.clone()));
  println!("Cheapest intersection: {}", get_cheapest(intersections));
}

fn get_closest(intersections: Vec<(i64, i64)>) -> i64 {
  intersections
    .into_iter()
    .min_by(
      |(x, _), (y, _)| if x > y { Ordering::Greater } else { Ordering::Less },
    )
    .map(|(x, _)| x)
    .unwrap()
}

fn get_cheapest(intersections: Vec<(i64, i64)>) -> i64 {
  intersections
    .into_iter()
    .min_by(
      |(_, x), (_, y)| if x > y { Ordering::Greater } else { Ordering::Less },
    )
    .map(|(_, x)| x)
    .unwrap()
}

fn get_intersections(paths: Vec<Vec<String>>) -> Vec<(i64, i64)> {
  let mut wire1_nodes: HashSet<(i64, i64)> = HashSet::new();
  let mut wire1_distance: HashMap<(i64, i64), i64> = HashMap::new();
  let mut coords = (0, 0);
  let mut counter = 0;

  for m in &paths[0] {
    let (direction, steps) = parse_movement(m);
    let (x, y) = coords;

    for s in 1..=steps {
      counter += 1;

      let step_coord = match direction {
        'L' => (x - s, y),
        'R' => (x + s, y),
        'U' => (x, y + s),
        'D' => (x, y - s),
        _ => panic!("invalid input"),
      };

      wire1_nodes.insert(step_coord);
      wire1_distance.insert(step_coord, counter);
    }
    coords = end_coords(coords, direction, steps);
  }

  let mut coords = (0, 0);
  let mut counter = 0;
  let mut intersections: Vec<(i64, i64)> = vec![];

  for m in &paths[1] {
    let (direction, steps) = parse_movement(m);
    let (x, y) = coords;

    for s in 1..=steps {
      counter += 1;

      let new_coords = match direction {
        'L' => (x - s, y),
        'R' => (x + s, y),
        'U' => (x, y + s),
        'D' => (x, y - s),
        _ => panic!("invalid input"),
      };

      if wire1_nodes.contains(&new_coords) {
        let (x, y) = new_coords;
        let wire1_counter = wire1_distance.get(&new_coords).unwrap();
        intersections.push((x.abs() + y.abs(), counter + wire1_counter));
      }
    }
    coords = end_coords(coords, direction, steps);
  }

  intersections
}

fn parse_movement(s: &str) -> (char, i64) {
  let direction = s.chars().next().expect("invalid direction");
  let steps = s
    .chars()
    .next()
    .map(|c| &s[c.len_utf8()..])
    .expect("invalid step")
    .parse::<i64>()
    .expect("invalid step");

  (direction, steps)
}

fn end_coords(start: (i64, i64), direction: char, steps: i64) -> (i64, i64) {
  let (x, y) = start;

  match direction {
    'L' => (x - steps, y),
    'R' => (x + steps, y),
    'U' => (x, y + steps),
    'D' => (x, y - steps),
    _ => panic!("invalid input"),
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn part1_1() {
    let data: Vec<Vec<String>> = vec![
      vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
      vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    ];
    let distance = super::get_closest(super::get_intersections(data));

    assert_eq!(distance, 159);
  }

  #[test]
  fn part1_2() {
    let data: Vec<Vec<String>> = vec![
      vec![
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53",
        "R51",
      ]
      .into_iter()
      .map(|s| s.to_string())
      .collect(),
      vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    ];
    let distance = super::get_closest(super::get_intersections(data));

    assert_eq!(distance, 135);
  }

  #[test]
  fn part2_1() {
    let data: Vec<Vec<String>> = vec![
      vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
      vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    ];
    let distance = super::get_cheapest(super::get_intersections(data));

    assert_eq!(distance, 610);
  }

  #[test]
  fn part2_2() {
    let data: Vec<Vec<String>> = vec![
      vec![
        "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53",
        "R51",
      ]
      .into_iter()
      .map(|s| s.to_string())
      .collect(),
      vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"]
        .into_iter()
        .map(|s| s.to_string())
        .collect(),
    ];
    let distance = super::get_cheapest(super::get_intersections(data));

    assert_eq!(distance, 410);
  }
}
