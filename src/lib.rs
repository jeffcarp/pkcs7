use std::iter::repeat;

pub fn pad(buffer: &mut Vec<u8>, block_size: usize) {
    let padding_size = block_size - (buffer.len() % block_size);
    buffer.extend(repeat(padding_size as u8).take(padding_size));
}

pub fn un_pad(buffer: &mut Vec<u8>) {
    if let Some(&pad_len) = buffer.last() {
        let len = buffer.len();
        buffer.truncate(len - pad_len as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkcs7_pads_short_input() {
        let mut actual: Vec<u8> = vec![8, 3, 4, 11, 4];
        pad(&mut actual, 8);

        let expected: Vec<u8> = vec![8, 3, 4, 11, 4, 3, 3, 3];
        assert_eq!(actual, expected);
    }

    #[test]
    fn pkcs7_un_pad_short_input() {
        let mut actual: Vec<u8> = vec![8, 3, 4, 11, 4, 3, 3, 3];
        un_pad(&mut actual);

        let expected: Vec<u8> = vec![8, 3, 4, 11, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn pecs7_pad_perfect_input() {
        let mut actual: Vec<u8> = vec![0, 1, 2, 3];
        pad(&mut actual, 4);

        let expected: Vec<u8> = vec![0, 1, 2, 3, 4, 4, 4, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn pkcs7_un_pad_perfect_input() {
        let mut actual: Vec<u8> = vec![0, 1, 2, 3, 4, 4, 4, 4];
        un_pad(&mut actual);

        let expected: Vec<u8> = vec![0, 1, 2, 3];
        assert_eq!(actual, expected);
    }
}
