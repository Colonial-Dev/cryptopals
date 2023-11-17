#![allow(dead_code)]

mod util;

use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    repeating_xor_crack()
}

fn repeating_xor_crack() -> Result<()> {
    let input = fs::read_to_string("src/data/1-6.txt")?;
    let input = input.lines().map(str::trim).collect::<String>();
    let input = util::b64_dec(input);

    let mut distances: Vec<_> = (2..=40)
        .map(|size| {
            let l = &input[0..size];
            let r = &input[size..size * 2];

            (size, util::bit_hamming(l, r) / size)
        })
        .collect();

    distances.sort_unstable_by_key(|(_, d)| *d);

    println!("{distances:?}");
    
    distances
        .iter()
        .take(4)
        .for_each(|(size, _)| {
            let chunks: Vec<_> = input.chunks_exact(*size).collect();
            let mut blocks = vec![vec![0_u8; *size]; *size];

            for (i, block) in blocks.iter_mut().enumerate() {
                chunks
                    .iter()
                    .map(|c| c[i])
                    .zip(block)
                    .for_each(|(b, t)| *t = b);
            }

            let key: Vec<_> = blocks
                .into_iter()
                .map(|block| {
                    (0..=255)
                        .map(|b| {
                            let out = util::xor_key(&block, [b]);
                            
                            (
                                util::score_text(&out),
                                b
                            )
                        })
                        .min_by(|a, b| {
                            a.0.partial_cmp(&b.0).unwrap()
                        })
                        .map(|(_, b)| b)
                        .unwrap()
                })
                .collect();

            let pt = util::xor_key(&input, key);
            println!("{}", std::str::from_utf8(&pt).unwrap_or("<invalid UTF8>"))
        });
        
    Ok(())
}