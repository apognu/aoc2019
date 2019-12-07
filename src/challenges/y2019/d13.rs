use std::{
  cmp::Ordering,
  collections::HashMap,
  fmt::{self, Display},
  io::{stdout, Write},
  thread,
  time::Duration,
};
use termion::{
  clear::AfterCursor,
  cursor::{DetectCursorPos, Goto, Hide},
  raw::IntoRawMode,
};

use crate::{util, util::intcode::Program};

type Coords = (u16, u16);
type Scene = HashMap<Coords, Object>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Object {
  Ball,
  Wall,
  Block,
  Paddle,
  Empty,
  GameOver,
}

impl From<i128> for Object {
  fn from(x: i128) -> Object {
    match x {
      0 => Object::Empty,
      1 => Object::Wall,
      2 => Object::Block,
      3 => Object::Paddle,
      4 => Object::Ball,
      _ => Object::GameOver,
    }
  }
}

impl Display for Object {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    match self {
      Object::Empty => write!(formatter, " "),
      Object::Wall => write!(formatter, "█"),
      Object::Block => write!(formatter, "★"),
      Object::Paddle => write!(formatter, "▂"),
      Object::Ball => write!(formatter, "●"),
      Object::GameOver => Ok(()),
    }
  }
}

struct Game {
  program: Program,
  scene: Scene,
  paddle: Option<u16>,
  score: i128,
  cursor: (u16, u16),
  print: bool,
}

impl Game {
  fn new(stack: Vec<i128>) -> Game {
    let mut stdout = stdout().into_raw_mode().unwrap();

    Game {
      program: Program::new(stack, vec![0]),
      scene: HashMap::new(),
      paddle: None,
      score: 0,
      cursor: stdout.cursor_pos().unwrap(),
      print: true,
    }
  }

  fn play(&mut self) {
    while !self.program.halted {
      if self.print {
        thread::sleep(Duration::from_micros(500));
      }
      let (x, y, value) = self.tick();

      if x == -1 && y == 0 {
        self.score = value;
        continue;
      }

      let object = value.into();

      match object {
        Object::GameOver => break,
        Object::Paddle => self.paddle = Some(x as u16),
        Object::Ball => {
          if let Some(paddle) = self.paddle {
            self.program.inputs = match paddle.cmp(&(x as u16)) {
              Ordering::Greater => vec![-1],
              Ordering::Less => vec![1],
              Ordering::Equal => vec![0],
            };
          }
        }
        _ => (),
      }
      self.scene.insert((x as u16, y as u16), object);

      if self.print {
        self.write_tile(x as u16, y as u16, object);
      }
    }

    if self.print {
      self.clear_scene();
    }
  }

  fn tick(&mut self) -> (i128, i128, i128) {
    (
      self.program.execute_for_output().unwrap(),
      self.program.execute_for_output().unwrap(),
      self.program.execute_for_output().unwrap(),
    )
  }

  fn write_tile(&self, x: u16, y: u16, object: Object) {
    print!("{}Score: {}", Goto(self.cursor.0, self.cursor.1), self.score);

    print!(
      "{}{}{}",
      Goto(self.cursor.0 + x, self.cursor.1 + y + 1),
      Hide,
      object
    );

    std::io::stdout().flush().unwrap();
  }

  fn clear_scene(&self) {
    print!("{}{}", Goto(self.cursor.0, self.cursor.1), AfterCursor);
  }
}

pub fn run() {
  let stack: Vec<i128> = util::read_split_file(2019, 13, ",")
    .iter()
    .map(|bit| bit.parse::<i128>().expect("invalid stack"))
    .collect();

  let mut game = Game::new(stack.clone());
  game.print = false;
  game.play();

  let blocks =
    game.scene.iter().filter(|(_, object)| **object == Object::Block).count();

  println!("There are {} blocks on the screen.", blocks);

  let mut game = Game::new(stack);
  game.program.stack[0] = 2;
  game.play();
  println!("Game over! Final score is {}.", game.score);
}
