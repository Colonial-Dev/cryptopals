/// Implementation of the PKCS#7 padding algorithm. Pads the provided input to `size` bytes.
pub fn pkcs7_pad(input: impl AsRef<[u8]>, size: u8) -> Box<[u8]> {
    let mut input = input.as_ref().to_vec();

    let mod_len = input.len() % (size as usize);
    let pad_len = (size as usize) - mod_len;

    input.extend( std::iter::once(pad_len as u8).cycle().take(pad_len) );

    input.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_pkcs7() {
        assert_eq!(
            pkcs7_pad(b"YELLOW SUBMARINE", 20).as_ref(),
            b"YELLOW SUBMARINE\x04\x04\x04\x04"
        );

        assert_eq!(
            pkcs7_pad(b"YELLOW SUBMARINE\x04\x04", 20).as_ref(),
            b"YELLOW SUBMARINE\x04\x04\x02\x02"
        );

        assert_eq!(
            pkcs7_pad(b"YELLOW SUBMARINE\x04\x04\x04", 20).as_ref(),
            b"YELLOW SUBMARINE\x04\x04\x04\x01"
        );
    }
}