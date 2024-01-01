#![allow(dead_code)]

mod util;

use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    ecb_detect()
}

/// Solution for challenge 1-3.
fn single_xor_crack() -> Result<()> {
    const INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    let input = util::hex_to_bytes(INPUT);

    let mut best_dec = None;
    let mut best_val = f32::MAX;

    for key in 0_u8..255 {
        let dec = util::xor_key(&input, [key]);
        let val = util::score_text(&dec);

        if val < best_val {
            best_val = val;
            best_dec = Some(dec);
        }
    }
    
    println!(
        "{}",
        String::from_utf8_lossy( &best_dec.unwrap() )
    );

    Ok(())
}

/// Solution for challenge 1-6.
fn repeating_xor_crack() -> Result<()> {
    let input = fs::read_to_string("src/data/1-6.txt")?
        .lines()
        .map(str::trim)
        .collect::<String>();

    let input = util::b64_dec(input);

    let candidate = (2..=40)
        .map(|size| {
            let mut dist = input
                .chunks_exact(size)
                .skip(1)
                .zip( input.chunks_exact(size) )
                .map(|(c, p)| util::bit_hamming(c, p) as f32 / size as f32)
                .sum::<f32>();

            dist /= input.chunks_exact(size).len() as f32;
            
            (size, dist)
        })
        .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2) )
        .map(|(s, _)| s)
        .expect("No candidate keysize found!");

    let mut blocks = vec![Vec::new(); candidate];

    for block in input.chunks_exact(candidate) {
        for (i, b) in block.iter().enumerate() {
            blocks[i].push(*b)
        }
    }

    let key: Vec<_> = blocks
        .into_iter()
        .map(|block| {
            let mut best_key = None;
            let mut best_val = f32::MAX;

            for key in 0_u8..128 {
                let dec = util::xor_key(&block, [key]);
                let val = util::score_text(dec);

                if val < best_val {
                    best_val = val;
                    best_key = Some(key);
                }
            }

            best_key.unwrap()
        })
        .collect();
    
    println!(
        "(Decrypted with key '{}')\n\n{}",
        String::from_utf8_lossy( &key.clone() ),
        String::from_utf8_lossy( &util::xor_key(&input, key) )
    );

    Ok(())
}

fn ecb_decrypt_example() -> Result<()> {
    use openssl::symm::{decrypt, Cipher};

    let input = fs::read_to_string("src/data/1-7.txt")?
        .lines()
        .map(str::trim)
        .collect::<String>();

    let input = util::b64_dec(input);

    let output = decrypt(
        Cipher::aes_128_ecb(),
        b"YELLOW SUBMARINE",
        None,
        &input
    )?;

    println!(
        "{}",
        String::from_utf8_lossy(&output)
    );

    Ok(())
}

fn ecb_detect() -> Result<()> {
    use std::collections::HashSet;

    let answer = fs::read_to_string("src/data/1-8.txt")?
        .lines()
        .map(util::hex_to_bytes)
        .find(|ct| {
            let mut set = HashSet::new();

            for block in ct.chunks_exact(16) {
                if set.contains(block) { return true; }
                else { set.insert(block); }
            }
            
            false
        })
        .expect("No ECB ciphertext detected!");

    println!("{:x?}", answer);

    Ok(())
}