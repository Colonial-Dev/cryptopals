#![allow(dead_code)]

mod util;

use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    repeating_xor_crack()
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
        .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap() )
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