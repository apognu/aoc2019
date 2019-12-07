use std::collections::HashMap;

use crate::{util, util::intcode::Program};

pub fn run() {
  let file = util::read_split_file(2019, 11, ",");
  let stack: Vec<i128> = file
    .iter()
    .map(|bit| bit.parse::<i128>().expect("invalid input"))
    .collect();

  let mut robot = PaintingRobot::new(stack.clone(), HashMap::default());
  robot.boot();

  println!("{} grid squares were painted", robot.painted_cells());

  let mut grid = HashMap::new();
  grid.insert((0, 0), Color::White);

  let mut robot = PaintingRobot::new(stack, grid);
  robot.boot();
  robot.report();
}

#[derive(Debug)]
struct PaintingRobot {
  program: Program,
  coords: Coords,
  facing: Direction,
  grid: HashMap<Coords, Color>,
}

impl PaintingRobot {
  fn new(stack: Vec<i128>, grid: HashMap<Coords, Color>) -> PaintingRobot {
    PaintingRobot {
      program: Program::new(stack, vec![]),
      coords: (0, 0),
      facing: Direction::Up,
      grid,
    }
  }

  fn boot(&mut self) {
    loop {
      self.program.inputs =
        vec![self.grid.get(&self.coords).unwrap_or(&Color::Black).value()];

      let color = match self.program.execute_for_output() {
        Some(0) => Color::Black,
        Some(1) => Color::White,
        _ => panic!("no color provided"),
      };
      let turn = match self.program.execute_for_output() {
        Some(0) => Turn::Left,
        Some(1) => Turn::Right,
        _ => panic!("no turn direction provided"),
      };

      if self.program.halted {
        break;
      }

      self.advance(color, turn);
    }
  }

  fn advance(&mut self, color: Color, turn: Turn) {
    self.grid.insert(self.coords, color);
    self.facing = self.facing.turn(turn);

    self.coords = match self.facing {
      Direction::Up => (self.coords.0, self.coords.1 + 1),
      Direction::Left => (self.coords.0 - 1, self.coords.1),
      Direction::Right => (self.coords.0 + 1, self.coords.1),
      Direction::Bottom => (self.coords.0, self.coords.1 - 1),
    };
  }

  fn painted_cells(&self) -> usize {
    self.grid.len()
  }

  fn report(&self) {
    self.get_printout(&mut std::io::stdout());
  }

  fn get_printout(&self, mut writer: impl std::io::Write) {
    let min_x = self.grid.keys().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = self.grid.keys().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = self.grid.keys().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = self.grid.keys().max_by_key(|(_, y)| y).unwrap().1;
    for y in (min_y..=max_y).rev() {
      for x in min_x..=max_x {
        let color = self.grid.get(&(x, y)).unwrap_or(&Color::Black);
        match color {
          Color::Black => write!(writer, " ").unwrap(),
          Color::White => write!(writer, "█").unwrap(),
        };
      }
      writeln!(writer).unwrap();
    }
  }
}

type Coords = (isize, isize);

#[derive(Debug, Copy, Clone)]
enum Direction {
  Up,
  Left,
  Bottom,
  Right,
}

impl From<isize> for Direction {
  fn from(angle: isize) -> Direction {
    match angle {
      0 | 360 => Direction::Up,
      90 => Direction::Right,
      180 => Direction::Bottom,
      270 | -90 => Direction::Left,
      _ => panic!("invalid turn angle"),
    }
  }
}

impl Direction {
  fn value(self) -> isize {
    match self {
      Direction::Up => 0,
      Direction::Right => 90,
      Direction::Bottom => 180,
      Direction::Left => 270,
    }
  }

  fn turn(self, turn: Turn) -> Direction {
    let angle = match turn {
      Turn::Left => self.value() - 90,
      Turn::Right => self.value() + 90,
    };

    angle.into()
  }
}

#[derive(Debug, PartialEq)]
enum Color {
  Black,
  White,
}

impl Color {
  fn value(&self) -> i128 {
    match self {
      Color::Black => 0,
      Color::White => 1,
    }
  }
}

#[derive(Debug, PartialEq)]
enum Turn {
  Left,
  Right,
}

#[cfg(test)]
mod tests {
  use super::{Color, PaintingRobot, Turn};
  use std::collections::HashMap;

  #[test]
  fn all() {
    let instructions = vec![
      (Color::White, Turn::Left),
      (Color::Black, Turn::Left),
      (Color::White, Turn::Left),
      (Color::White, Turn::Left),
      (Color::Black, Turn::Right),
      (Color::White, Turn::Left),
      (Color::White, Turn::Left),
    ];

    let mut robot = PaintingRobot::new(vec![], HashMap::default());

    for (color, turn) in instructions {
      robot.advance(color, turn);
    }

    assert_eq!(robot.painted_cells(), 6);

    let mut result = Vec::new();
    robot.get_printout(&mut result);

    assert_eq!(result, "  █\n  █\n██ \n".as_bytes());
  }
}
