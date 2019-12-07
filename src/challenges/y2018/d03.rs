use regex::Regex;
use std::collections::HashSet;

use crate::util;

#[derive(Debug, PartialEq, Clone)]
struct Fabric {
  id: String,
  x1: usize,
  x2: usize,
  y1: usize,
  y2: usize,
}

pub fn run() {
  let rgx = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
  let file = util::read_file_by_lines(2018, 3);

  let fabrics: Vec<Fabric> = file
    .iter()
    .map(|fabric| {
      let captures = rgx.captures(&fabric).unwrap();

      let x =
        captures.get(2).map(|x| x.as_str().parse::<usize>().unwrap()).unwrap();
      let y =
        captures.get(3).map(|y| y.as_str().parse::<usize>().unwrap()).unwrap();

      Fabric {
        id: captures.get(1).unwrap().as_str().to_owned(),
        x1: x,
        x2: x
          + captures
            .get(4)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap()
          - 1,
        y1: y,
        y2: y
          + captures
            .get(5)
            .map(|x| x.as_str().parse::<usize>().unwrap())
            .unwrap()
          - 1,
      }
    })
    .collect();

  let overlaps = find_overlapping_inches(&fabrics);

  println!("There are {} inches of fabric overlapping.", overlaps.len());

  let intact = find_overlapping_fabrics(&fabrics, &overlaps).unwrap();

  println!("Claim not overlapping with any other: {}", intact.id);
}

fn find_overlapping_inches(fabrics: &[Fabric]) -> HashSet<(usize, usize)> {
  let mut overlaps: HashSet<(usize, usize)> = HashSet::new();
  let mut grid: HashSet<(usize, usize)> = HashSet::new();

  for fabric in fabrics {
    for y in fabric.y1..=fabric.y2 {
      for x in fabric.x1..=fabric.x2 {
        if grid.contains(&(x, y)) {
          overlaps.insert((x, y));
        }

        grid.insert((x, y));
      }
    }
  }

  overlaps
}

fn find_overlapping_fabrics(
  fabrics: &[Fabric], inches: &HashSet<(usize, usize)>,
) -> Option<Fabric> {
  'fabric: for fabric in fabrics {
    for y in fabric.y1..=fabric.y2 {
      for x in fabric.x1..=fabric.x2 {
        if inches.contains(&(x, y)) {
          continue 'fabric;
        }
      }
    }

    return Some(fabric.clone());
  }

  None
}
