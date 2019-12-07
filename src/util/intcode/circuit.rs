use super::Program;

pub struct Circuit<F> {
  programs: Vec<Program>,
  inputs: F,
  feedback: bool,
}

impl<F> Circuit<F>
where
  F: Fn(usize, usize, i128) -> Vec<i128>,
{
  pub fn with_copies(
    count: usize, stack: Vec<i128>, inputs: F, feedback: bool,
  ) -> Self {
    let programs =
      (0..count).map(|_| Program::new(stack.clone(), vec![])).collect();

    Self { programs, inputs, feedback }
  }

  pub fn execute(&mut self) -> i128 {
    let mut cycle = 0;
    let mut output = 0;
    loop {
      for (index, program) in &mut self.programs.iter_mut().enumerate() {
        program.inputs = (self.inputs)(cycle, index, output);

        match program.execute_for_output() {
          Some(retval) => output = retval,
          None => panic!("program had no output"),
        }
      }

      if !self.feedback || self.programs[self.programs.len() - 1].halted {
        return output;
      }

      cycle += 1;
    }
  }
}
