/// XORs two equal-length byte buffers together, producing a third output buffer.
/// 
/// This function panics if the buffers are not equally sized.
pub fn xor_buf(l: impl AsRef<[u8]>, r: impl AsRef<[u8]>) -> Box<[u8]> {
    let l = l.as_ref();
    let r = r.as_ref();

    assert_eq!(
        l.len(),
        r.len()
    );

    let mut out = Vec::with_capacity(
        l.len()
    );

    for (l, r) in l.iter().zip(r) {
        out.push(l ^ r)
    }

    out.into_boxed_slice()
}

/// XOR a byte buffer against a given key. The key will be truncated or repeated to equal the buffer size.
pub fn xor_key(buf: impl AsRef<[u8]>, key: impl AsRef<[u8]>) -> Box<[u8]> {
    let buf = buf.as_ref();
    let key = key.as_ref();

    let pad_key: Vec<_> = key
        .iter()
        .copied()
        .cycle()
        .take(buf.len())
        .collect();

    xor_buf(
        buf,
        pad_key
    )
}

/// Compute the bitwise Hamming distance between two equally-sized byte buffers.
/// 
/// This function panics if the buffers are not equally sized.
pub fn bit_hamming(l: impl AsRef<[u8]>, r: impl AsRef<[u8]>) -> usize {
    let l = l.as_ref();
    let r = r.as_ref();

    assert_eq!(
        l.len(),
        r.len()
    );

    l
        .iter()
        .zip(r)
        .map(|(x, y)| (x ^ y).count_ones() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util::*;

    #[test]
    fn fixed_xor() {
        let buf = hex_to_bytes("1c0111001f010100061a024b53535009181c");
        let key = hex_to_bytes("686974207468652062756c6c277320657965");

        assert_eq!(
            hex_to_bytes("746865206b696420646f6e277420706c6179"),
            xor_buf(buf, key)
        )
    }

    #[test]
    fn repeating_xor() {
        let data = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";

        let out = xor_key(
            data,
            key
        );

        assert_eq!(
            hex_to_bytes("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272\na282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"),
            out
        )
    }

    #[test]
    fn hamming() {
        let l = b"this is a test";
        let r = b"wokka wokka!!!";

        assert_eq!(
            37,
            bit_hamming(l, r)
        );
    }
}