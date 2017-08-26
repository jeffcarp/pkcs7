pub fn pad(vec: &mut Vec<u8>, block_size: usize) -> &mut Vec<u8> {
    let remainder = vec.len() % block_size;
    let mut padding_size = block_size - remainder;
    let padding_byte = padding_size as u8;

    if remainder > 0 {
        while padding_size > 0 {
            vec.push(padding_byte);
            padding_size -= 1;
        }
    }

    return vec;
}


#[cfg(test)]
mod tests {

    use super::pad;

    #[test]
    fn pkcs7_pads_short_input() {
        let mut actual: Vec<u8> = vec![8, 3, 4, 11, 4];
        let block_size = 8;
        pad(&mut actual, block_size);

        let expected: Vec<u8> = vec![8, 3, 4, 11, 4, 3, 3, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn pkcs7_doesnt_pad_perfect_length_input() {
        let mut actual: Vec<u8> = vec![0, 1, 2, 3];
        pad(&mut actual, 4);

        let expected: Vec<u8> = vec![0, 1, 2, 3];
        assert_eq!(actual, expected);
    }

}
