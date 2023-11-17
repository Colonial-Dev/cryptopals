// From https://en.wikipedia.org/wiki/Letter_frequency
const FREQUENCIES: [f32; 26] = [
    8.2,
    1.5,
    2.8,
    4.3,
    12.7,
    2.2,
    2.0,
    6.1,
    7.0,
    0.15,
    0.77,
    4.0,
    2.4,
    6.7,
    7.5,
    1.9,
    0.095,
    6.0,
    6.3,
    9.1,
    2.8,
    0.98,
    2.4,
    0.15,
    2.0,
    0.074,
];

pub fn score_text(input: impl AsRef<[u8]>) -> f32 {
    let mut occurrences = [0_u32; 27];
    let input = input.as_ref();

    input
        .iter()
        .copied()
        .for_each(|b| match b {
            b'A'..=b'Z' => occurrences[(b - b'A') as usize] += 1,
            b'a'..=b'z' => occurrences[(b - b'a') as usize] += 1,
            _ => ()
        });
    
    occurrences
        .iter()
        .map(|n| {
            (*n as f32) / input.len() as f32
        })
        .zip(FREQUENCIES)
        .map(|(actual, expected)| actual - expected)
        .map(f32::abs)
        .sum()
}