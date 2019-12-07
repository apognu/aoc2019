#[derive(Debug, Copy, Clone)]
pub(super) enum Mode {
  Immediate,
  Position,
  Relative,
}

impl From<i128> for Mode {
  fn from(mode: i128) -> Self {
    match mode {
      0 => Mode::Position,
      1 => Mode::Immediate,
      2 => Mode::Relative,
      _ => panic!("invalid mode"),
    }
  }
}
