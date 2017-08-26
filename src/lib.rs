use std::iter::repeat;

pub fn pad(buffer: &mut Vec<u8>, block_size: u8) {
    let block_size = block_size as usize;

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

    #[test]
    fn pkcs7_u8_overflow() {
        const BLOCK_SIZE: usize = 11;

        let expected = vec![0; 1_000_000];

        let mut actual = expected.clone();
        pad(&mut actual, BLOCK_SIZE as u8);

        assert_eq!(actual.len() % BLOCK_SIZE, 0);

        un_pad(&mut actual);

        assert_eq!(actual, expected);
    }

    #[test]
    fn pkcs7_stackexchange_1() {
        let expected = [
            0x48,
            0x65,
            0x6C,
            0x6C,
            0x6F,
            0x2C,
            0x20,
            0x57,
            0x6F,
            0x72,
            0x6C,
            0x64,
            0x21,
        ];

        let mut buffer = expected.to_vec();
        pad(&mut buffer, 16);

        assert_eq!(
            buffer.as_slice(),
            [
                0x48,
                0x65,
                0x6C,
                0x6C,
                0x6F,
                0x2C,
                0x20,
                0x57,
                0x6F,
                0x72,
                0x6C,
                0x64,
                0x21,
                0x03,
                0x03,
                0x03
            ].as_ref()
        );

        un_pad(&mut buffer);
        assert_eq!(buffer.as_slice(), expected.as_ref());
    }

    #[test]
    fn pkcs7_stackexchange_2() {
        let expected = [
            0x48,
            0x65,
            0x6C,
            0x6C,
            0x6F,
            0x2C,
            0x20,
            0x57,
            0x6F,
            0x72,
            0x6C,
            0x64,
            0x21,
        ];

        let mut buffer = expected.to_vec();
        pad(&mut buffer, 20);

        assert_eq!(
            buffer.as_slice(),
            [
                0x48,
                0x65,
                0x6C,
                0x6C,
                0x6F,
                0x2C,
                0x20,
                0x57,
                0x6F,
                0x72,
                0x6C,
                0x64,
                0x21,
                0x07,
                0x07,
                0x07,
                0x07,
                0x07,
                0x07,
                0x07
            ].as_ref()
        );

        un_pad(&mut buffer);
        assert_eq!(buffer.as_slice(), expected.as_ref());
    }
}
