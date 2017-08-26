//! # Example
//!
//! ```rust
//! # extern crate pkcs7;
//! const BLOCK_SIZE: usize = 16;
//!
//! let un_padded = b"This is a test.";
//!
//! let mut buffer = un_padded.to_vec();
//! pkcs7::pad(&mut buffer, BLOCK_SIZE as u8);
//!
//! assert_eq!(buffer.len() % BLOCK_SIZE, 0);
//!
//! pkcs7::un_pad(&mut buffer);
//!
//! assert_eq!(buffer.as_slice(), un_padded);
//! ```

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
