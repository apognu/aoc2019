use std::collections::HashSet;

use crate::{util, util::intcode::Program};

#[derive(Copy, Clone)]
enum Direction {
  North,
  South,
  West,
  East,
}

impl From<Option<&i128>> for Direction {
  fn from(direction: Option<&i128>) -> Direction {
    match direction {
      Some(1) => Direction::North,
      Some(2) => Direction::South,
      Some(3) => Direction::West,
      Some(4) => Direction::East,
      _ => panic!("invalid direction"),
    }
  }
}

impl Direction {
  fn all() -> [Direction; 4] {
    [Direction::North, Direction::South, Direction::West, Direction::East]
  }

  fn to_input(self) -> i128 {
    match self {
      Direction::North => 1,
      Direction::South => 2,
      Direction::West => 3,
      Direction::East => 4,
    }
  }

  fn reverse(self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
      Direction::East => Direction::West,
    }
  }
}

struct RepairRobot {
  position: (isize, isize),
  tank: (isize, isize),
  distance: u32,
  program: Program,
  visited: HashSet<(isize, isize)>,
}

impl RepairRobot {
  fn new(stack: Vec<i128>) -> RepairRobot {
    RepairRobot {
      position: (0, 0),
      program: Program::new(stack, vec![]),
      tank: (0, 0),
      distance: 0,
      visited: HashSet::new(),
    }
  }

  fn find_oxygen_tank(&mut self) {
    let mut path: Vec<Direction> = vec![];
    let mut distance = 0;

    loop {
      self.visited.insert(self.position);

      let mut moved = false;

      if !self.program.inputs.is_empty() {
        distance -= 1;
        let next = match self.program.inputs.get(0).into() {
          Direction::North => (self.position.0, self.position.1 - 1),
          Direction::South => (self.position.0, self.position.1 + 1),
          Direction::West => (self.position.0 - 1, self.position.1),
          Direction::East => (self.position.0 + 1, self.position.1),
        };

        self.position = next;
        self.program.execute();
        continue;
      }

      for direction in Direction::all().iter() {
        let next = match direction {
          Direction::North => (self.position.0, self.position.1 - 1),
          Direction::South => (self.position.0, self.position.1 + 1),
          Direction::West => (self.position.0 - 1, self.position.1),
          Direction::East => (self.position.0 + 1, self.position.1),
        };

        if self.visited.contains(&next) {
          continue;
        }

        self.program.inputs.push(direction.to_input());

        match self.program.execute_for_output() {
          Some(0) => continue,
          Some(1) => {
            distance += 1;
            self.position = next;

            path.push(*direction);
            moved = true;
            break;
          }

          Some(2) => {
            distance += 1;
            self.tank = self.position;
            self.distance = distance;
            self.position = next;

            path.push(*direction);
            moved = true;
            break;
          }
          _ => panic!("unknown output"),
        }
      }

      if moved {
        continue;
      }

      match path.pop() {
        Some(direction) => {
          self.program.inputs.push(direction.reverse().to_input())
        }
        _ => return,
      }
    }
  }
}

pub fn run() {
  let stack: Vec<i128> = util::read_split_file(2019, 15, ",")
    .iter()
    .map(|bit| bit.parse::<i128>().unwrap())
    .collect();

  let mut robot = RepairRobot::new(stack);
  robot.find_oxygen_tank();

  println!("Distance to oxygen tank: {}", robot.distance);
  println!("Time taken to fill the ship with oxygen: {}", fill_ship(&robot));
}

fn fill_ship(robot: &RepairRobot) -> i32 {
  let mut visited: HashSet<(isize, isize)> = HashSet::new();
  let mut position = robot.tank;
  let mut path: Vec<(isize, isize)> = vec![];
  let (mut time, mut result) = (0, 0);

  'main: loop {
    visited.insert(position);

    for direction in Direction::all().iter() {
      let next = match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::South => (position.0, position.1 + 1),
        Direction::West => (position.0 - 1, position.1),
        Direction::East => (position.0 + 1, position.1),
      };

      if !robot.visited.contains(&next) || visited.contains(&next) {
        continue;
      }

      path.push(position);

      time += 1;
      if time >= result {
        result += 1;
      }

      position = next;
      continue 'main;
    }

    time -= 1;
    match path.pop() {
      Some(back) => position = back,
      None => return result,
    }
  }
}
