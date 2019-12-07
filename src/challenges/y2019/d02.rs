use crate::{util, util::intcode::Program};

pub fn run() {
  let file = util::read_split_file(2019, 2, ",");
  let stack: Vec<i128> = file
    .iter()
    .map(|bit| bit.parse::<i128>().expect("invalid input"))
    .collect();

  let mut program = Program::new(stack.clone(), vec![]);

  program.stack[1] = 12;
  program.stack[2] = 2;
  program.execute();

  println!("Gravity assist result: {}", program.stack[0]);

  for noun in 0..=99 {
    for verb in 0..=99 {
      let mut program = Program::new(stack.clone(), vec![]);

      program.stack[1] = noun;
      program.stack[2] = verb;

      program.execute();

      if program.stack[0] == 19_690_720 {
        println!("Gravity assist parameters: {}", 100 * noun + verb);
        return;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::util::intcode::Program;
  #[test]
  fn part1() {
    let data = vec![
      (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
      (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
      (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
      (vec![1, 1, 1, 4, 99, 5, 6, 0, 99], vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
    ];

    for (stack, result) in data {
      let mut program = Program::new(stack, vec![]);
      program.execute();

      assert_eq!(program.stack, result);
    }
  }
}
