pub fn pad(vec: &mut Vec<u8>, block_size: usize) -> &mut Vec<u8> {
  let remainder = vec.len() % block_size;
  let padding_size = block_size - remainder;
  let padding_byte = padding_size as u8;

  for i in 0..padding_size {
    vec.push(padding_byte);
  }

  return vec
}


#[cfg(test)]
mod tests {

  use super::pad;

  #[test]
  fn pkcs7_pads_short_input() {
    let mut actual: Vec<u8> = vec![0, 1, 2, 3];
    let block_size = 8;
    pad(&mut actual, block_size);

    let expected: Vec<u8> = vec![0, 1, 2, 3, 4, 4, 4, 4];
    assert_eq!(actual, expected);
  }

  #[test]
  fn pkcs7_doesnt_pad_perfect_length_input() {
    let mut actual: Vec<u8> = vec![0, 1, 2, 3];
    pad(&mut actual, 4);

    let expected: Vec<u8> = vec![0, 1, 2, 3, 4, 4, 4, 4];
    assert_eq!(actual, expected);
  }

}
