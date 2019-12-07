use crate::util;

const CANVAS_SIZE: (usize, usize) = (25, 6);

pub fn run() {
  let file = util::read_chared_file(2019, 8);
  let pixels: Vec<u8> = file
    .iter()
    .map(|c| c.to_string().parse::<u8>().expect("invalid input"))
    .collect();

  let (width, height) = CANVAS_SIZE;
  let layers = get_layers(pixels, width, height);
  let checksum = checksum(&layers);

  println!("Image checksum: {}", checksum);

  println!("Image render:");
  render(&layers, width, height);
}

fn get_layers(raw: Vec<u8>, width: usize, height: usize) -> Vec<Vec<Vec<u8>>> {
  let layer_count = raw.len() as u32 / (width * height) as u32;
  let layers: Vec<Vec<Vec<u8>>> = raw
    .chunks(raw.len() / layer_count as usize)
    .map(|layer| {
      layer.to_vec().chunks(width as usize).map(|x| x.to_vec()).collect()
    })
    .collect();

  layers
}

fn checksum(layers: &[Vec<Vec<u8>>]) -> u32 {
  let min_zeroes = layers.iter().min_by_key(|layer| {
    layer.iter().fold(0, |acc, row| {
      acc + row.iter().fold(0, |acc, x| if x == &0 { acc + 1 } else { acc })
    })
  });
  let (ones, twos) = min_zeroes.into_iter().flatten().flatten().fold(
    (0, 0),
    |(ones, twos), x| {
      if x == &1 {
        (ones + 1, twos)
      } else if x == &2 {
        (ones, twos + 1)
      } else {
        (ones, twos)
      }
    },
  );

  ones * twos
}

fn render(layers: &[Vec<Vec<u8>>], width: usize, height: usize) {
  get_printout(layers, width, height, &mut std::io::stdout());
}

fn get_printout(
  layers: &[Vec<Vec<u8>>], width: usize, height: usize,
  mut writer: impl std::io::Write,
) {
  for y in 0..height {
    for x in 0..width {
      for layer in layers {
        let color = layer[y as usize][x as usize];
        if color == 0 {
          write!(writer, " ").unwrap();
          break;
        } else if color == 1 {
          write!(writer, "█").unwrap();
          break;
        }
      }
    }

    writeln!(writer).unwrap();
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn all() {
    let (width, height) = (2, 2);
    let raw = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
    let layers = super::get_layers(raw, 3, 2);

    assert_eq!(
      layers,
      vec![
        vec![vec![1, 2, 3], vec![4, 5, 6]],
        vec![vec![7, 8, 9], vec![0, 1, 2]]
      ]
    );
    assert_eq!(1, super::checksum(&layers));

    let mut result = Vec::new();
    super::get_printout(&layers, width, height, &mut result);

    assert_eq!(result, "█\n █\n".as_bytes());
  }
}
