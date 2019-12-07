macro_rules! opcodes {
  ( $( $name:ident << $code:expr ),* ) => {
    use super::mode::Mode;

    #[derive(Debug, Copy, Clone)]
    enum Opcode {
      $(
        $name(Mode, Mode, Mode)
      ),*
    }

    impl From<i128> for Opcode {
      fn from(code: i128) -> Self {
        let modes = (code / 100 % 10, code / 1000 % 10, code / 10000 % 10);
        match code % 100 {
          $(
            $code => Opcode::$name(modes.0.into(), modes.1.into(), modes.2.into()),
          )*
          _ => panic!("invalid opcode"),
        }
      }
    }

    use Opcode::*;
  };
}
