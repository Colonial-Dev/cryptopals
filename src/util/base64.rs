// From https://en.wikipedia.org/wiki/Base64
const B64_LUT: [&str; 64] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "+", "/"
];

const B64_PAD: u8 = b'=';

/// Encode a byte buffer as RFC 4648 padded Base64.
pub fn b64_enc(input: impl AsRef<[u8]>) -> String {
    let input = input.as_ref();

    #[allow(clippy::precedence)]
    let mut out = String::with_capacity(
        (4 * input.len() / 3) + 3 & !3
    );

    input
        .chunks(3)
        .for_each(|chunk| {
            let fetch = |index| {
                chunk
                    .get(index)
                    .copied()
                    .unwrap_or(0) as u32
            };

            let a = fetch(0);
            let b = fetch(1);
            let c = fetch(2);

            let n = (a << 16) + (b << 8) + c;

            out += B64_LUT[(n >> 18 & 63) as usize];
            out += B64_LUT[(n >> 12 & 63) as usize];
            out += B64_LUT[(n >> 6 & 63)  as usize];
            out += B64_LUT[(n & 63)       as usize];
        });

    let out_len = out.len();
    
    // SAFETY: our B64 character set is pure ASCII, so we can ignore UTF-8 concerns
    // and just directly edit the underlying byte array.
    match input.len() % 3 {
        0 => (),
        1 => unsafe {
            out.as_bytes_mut()[out_len - 1] = B64_PAD;
            out.as_bytes_mut()[out_len - 2] = B64_PAD;
        },
        2 => unsafe {
            out.as_bytes_mut()[out_len - 1] = B64_PAD
        },
        _ => unreachable!()
    }

    out
}

/// Decode a RFC 4648 padded Base64 string into a byte buffer.
/// 
/// This function panics if the input string is not valid Base64.
pub fn b64_dec(input: impl AsRef<[u8]>) -> Box<[u8]> {
    let mut out = Vec::new();
    let input = input.as_ref();

    input
        .chunks_exact(4)
        .for_each(|chunk| {
            let fetch = |index| {
                let byte = chunk[index];

                let idx = match byte {
                    b'A'..=b'Z' => byte - b'A',
                    b'a'..=b'z' => (byte - b'a') + 26,
                    b'0'..=b'9' => (byte - b'0') + 52,
                    b'+' => 62,
                    b'/' => 63,
                    b'=' => 0,
                    _ => panic!("Encountered invalid Base64 ({} {})", byte, byte as char)
                };

                usize::from(idx)
            };

            let a = fetch(0);
            let b = fetch(1);
            let c = fetch(2);
            let d = fetch(3);

            let n = (a << 18) + (b << 12) + (c << 6) + d;

            let a = (n >> 16) & 0xFF;
            let b = (n >> 8) & 0xFF;
            let c = n & 0xFF;

            out.push(a as u8);
            out.push(b as u8);
            out.push(c as u8);
        });

    input
        .iter()
        .rev()
        .take_while(|b| **b == b'=')
        .for_each(|_| {
            let _ = out.pop();
        });

    out.into_boxed_slice()
}

pub fn hex_to_bytes(input: impl AsRef<str>) -> Box<[u8]> {
    let input = input.as_ref();

    let mut hex_bytes = input
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        })
        .fuse();

    let mut bytes = Vec::with_capacity(input.len() / 2);

    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push(h << 4 | l)
    }

    bytes.into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_b64() {
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            b64_enc(hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"))
        );

        assert_eq!(
            "VGhlIHN0YXJzLCBsaWtlIGR1c3Q=",
            b64_enc("The stars, like dust")
        )
    }

    #[test]
    fn roundtrip_b64() {
        assert_eq!(
            b"The stars, like dust.".to_vec().into_boxed_slice(),
            b64_dec(b64_enc("The stars, like dust."))
        );

        assert_eq!(
            b"The stars, like dust".to_vec().into_boxed_slice(),
            b64_dec(b64_enc("The stars, like dust"))
        );

        assert_eq!(
            b"The stars, like dus".to_vec().into_boxed_slice(),
            b64_dec(b64_enc("The stars, like dus"))
        );
    }
}