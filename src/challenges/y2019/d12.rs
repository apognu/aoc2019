use num::integer::lcm;
use regex::Regex;
use std::{
  cmp::Ordering,
  collections::{hash_map::DefaultHasher, HashSet},
  hash::{Hash, Hasher},
  slice::{Iter, IterMut},
};

use crate::util;

const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;

type SystemStates = [HashSet<u64>; 3];
type Point = [isize; 3];

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Moon {
  position: Point,
  velocity: Point,
}

#[derive(Debug, Clone)]
struct System(Vec<Moon>);

impl System {
  fn iter(&self) -> Iter<Moon> {
    self.0.iter()
  }

  fn iter_mut(&mut self) -> IterMut<Moon> {
    self.0.iter_mut()
  }

  fn generator(&mut self) -> SystemGenerator {
    SystemGenerator { state: self, epoch: 0 }
  }

  fn energy(&self) -> isize {
    self
      .iter()
      .map(|moon| {
        let potential: isize = moon.position.iter().map(|p| p.abs()).sum();
        let kinetic: isize = moon.velocity.iter().map(|p| p.abs()).sum();

        potential * kinetic
      })
      .sum()
  }

  fn apply_gravity(&mut self) {
    let system = self.clone();

    for moon in &mut self.iter_mut() {
      for m in system.iter() {
        if m == moon {
          continue;
        }

        (0..3).for_each(|axis| {
          moon.velocity[axis] = match moon.position[axis].cmp(&m.position[axis])
          {
            Ordering::Greater => moon.velocity[axis] - 1,
            Ordering::Less => moon.velocity[axis] + 1,
            Ordering::Equal => moon.velocity[axis],
          };
        });
      }
    }
  }

  fn apply_velocity(&mut self) {
    for moon in self.iter_mut() {
      moon.position = [
        moon.position[X] + moon.velocity[X],
        moon.position[Y] + moon.velocity[Y],
        moon.position[Z] + moon.velocity[Z],
      ];
    }
  }

  fn hash(&self) -> [u64; 3] {
    let state: Vec<u64> = (0..3)
      .map(|axis| {
        let mut hasher = DefaultHasher::new();
        let system = self
          .iter()
          .map(move |moon| (moon.position[axis], moon.velocity[axis]))
          .collect::<Vec<(isize, isize)>>();

        system.hash(&mut hasher);
        hasher.finish()
      })
      .collect();

    [*state.get(0).unwrap(), *state.get(1).unwrap(), *state.get(2).unwrap()]
  }

  fn full_circle_at(&mut self) -> usize {
    let mut states: SystemStates = Default::default();
    let mut results: [u128; 3] = [0, 0, 0];

    for (epoch, system) in self.generator() {
      let hashes = system.hash();

      (0..3).for_each(|axis| {
        if states[axis].contains(&hashes[axis]) && results[axis] == 0 {
          results[axis] = epoch;
        }
      });

      if results.iter().all(|result| *result > 0) {
        break;
      }

      (0..3).for_each(|axis| {
        states[axis].insert(hashes[axis]);
      });
    }

    lcm(lcm(results[X], results[Y]), results[Z]) as usize
  }
}

impl From<Vec<Moon>> for System {
  fn from(system: Vec<Moon>) -> System {
    System(system)
  }
}

struct SystemGenerator<'a> {
  state: &'a mut System,
  epoch: u128,
}

impl<'a> Iterator for SystemGenerator<'a> {
  type Item = (u128, System);

  fn next(&mut self) -> Option<Self::Item> {
    let system = match self.epoch {
      0 => Some((self.epoch, self.state.clone())),
      _ => {
        self.state.apply_gravity();
        self.state.apply_velocity();
        Some((self.epoch, self.state.clone()))
      }
    };

    self.epoch += 1;

    system
  }
}

pub fn run() {
  let rgx = Regex::new(r"<x=(.+), y=(.+), z=(.+)>").unwrap();
  let file = util::read_file_by_lines(2019, 12);

  let mut system: System = file
    .iter()
    .map(|line| {
      let captures = rgx.captures(&line).unwrap();

      if rgx.captures_len() != 4 {
        panic!();
      }

      let coords: Vec<isize> = captures
        .iter()
        .skip(1)
        .map(|x| x.unwrap().as_str().parse::<isize>().unwrap())
        .collect();

      Moon {
        position: [
          *coords.get(X).unwrap(),
          *coords.get(Y).unwrap(),
          *coords.get(Z).unwrap(),
        ],
        ..Default::default()
      }
    })
    .collect::<Vec<Moon>>()
    .into();

  let (_, system_1000th) = system.clone().generator().nth(1000).unwrap();
  println!(
    "Total systemic energy after 1000 steps: {}",
    system_1000th.energy()
  );

  println!("First epoch at which a state repeats: {}", system.full_circle_at());
}
