use crate::util::{self, intcode::Program};

pub fn run() {
  let file = util::read_split_file(2019, 5, ",");
  let stack: Vec<i128> = file
    .iter()
    .map(|bit| bit.parse::<i128>().expect("invalid input"))
    .collect();

  let mut program = Program::new(stack, vec![5]);

  println!("Result: {:?}", program.execute_for_output().unwrap());
}

#[cfg(test)]
mod tests {
  use crate::util::intcode::Program;

  #[test]
  fn part2_1() {
    let data = vec![
      (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8, 1), // Equal to 8
      (vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 10, 0), // Different than 8
      (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 5, 1), // Less than 8
      (vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 10, 0), // Greather than 8
      (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8, 1), // Equal to 8 (immediate)
      (vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 20, 0), // Different than 8 (immediate)
      (vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 0, 0), // Equal to 0 (jump)
      (vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 10, 1), // Different than 0 (jump)
      (
        vec![
          3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
          1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
          999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ],
        5,
        999,
      ),
      (
        vec![
          3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
          1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
          999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ],
        12,
        1001,
      ),
      (
        vec![
          3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
          1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
          999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ],
        8,
        1000,
      ),
    ];

    for (stack, input, ret) in data {
      let mut program = Program::new(stack, vec![input]);

      assert_eq!(program.execute_for_output(), Some(ret));
    }
  }
}
