use crate::{util, util::intcode::Program};

pub fn run() {
  let file = util::read_split_file(2019, 9, ",");
  let stack: Vec<i128> = file
    .iter()
    .map(|bit| bit.parse::<i128>().expect("invalid input"))
    .collect();

  let mut program = Program::new(stack.clone(), vec![1]);
  println!("BOOST keycode: {}", program.execute_for_output().unwrap());
  let mut program = Program::new(stack, vec![2]);
  println!("BOOST distress signal: {}", program.execute_for_output().unwrap());
}

#[cfg(test)]
mod tests {
  use crate::util::intcode::Program;

  #[test]
  fn part1() {
    let stack = vec![
      109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut program = Program::new(stack.clone(), vec![]);
    program.execute();

    assert_eq!(program.stack, stack);

    let mut program =
      Program::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], vec![]);

    assert_ne!(program.execute_for_output().unwrap() / 1000000000000000, 0);

    let mut program = Program::new(vec![104, 1125899906842624, 99], vec![]);

    assert_eq!(program.execute_for_output().unwrap(), 1125899906842624);
  }
}
