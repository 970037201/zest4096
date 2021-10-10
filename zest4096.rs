use std::num::Wrapping;

#[inline(always)]
fn rotl(a: u64, b: u8) -> u64 {
    a << b | a >> (64 - b)
}

#[inline(always)]
fn eighth_round(e: (usize, usize, usize, usize, usize, usize, usize, usize), x: &mut [u64; 64]) {
    x[e.0] = (Wrapping(x[e.0]) + Wrapping(x[e.1])).0;
    x[e.7] = rotl(x[e.7] ^ x[e.0], 32);
    x[e.2] = (Wrapping(x[e.2]) + Wrapping(x[e.3])).0;
    x[e.1] = rotl(x[e.1] ^ x[e.2], 28);
    x[e.4] = (Wrapping(x[e.4]) + Wrapping(x[e.5])).0;
    x[e.3] = rotl(x[e.3] ^ x[e.4], 24);
    x[e.6] = (Wrapping(x[e.6]) + Wrapping(x[e.7])).0;
    x[e.5] = rotl(x[e.5] ^ x[e.6], 21);

    x[e.0] = (Wrapping(x[e.0]) + Wrapping(x[e.1])).0;
    x[e.7] = rotl(x[e.7] ^ x[e.0], 16);
    x[e.2] = (Wrapping(x[e.2]) + Wrapping(x[e.3])).0;
    x[e.1] = rotl(x[e.1] ^ x[e.2], 12);
    x[e.4] = (Wrapping(x[e.4]) + Wrapping(x[e.5])).0;
    x[e.3] = rotl(x[e.3] ^ x[e.4], 8);
    x[e.6] = (Wrapping(x[e.6]) + Wrapping(x[e.7])).0;
    x[e.5] = rotl(x[e.5] ^ x[e.6], 5);
}

pub fn zest_hash(input: [u64; 64]) -> [u64; 64] {
    let mut x = input;
    for _ in 0..48 {
        // Odd round (columns)
        eighth_round((0, 8, 16, 24, 32, 40, 48, 56), &mut x);
        eighth_round((1, 9, 17, 25, 33, 41, 49, 57), &mut x);
        eighth_round((2, 10, 18, 26, 34, 42, 50, 58), &mut x);
        eighth_round((3, 11, 19, 27, 35, 43, 51, 59), &mut x);
        eighth_round((4, 12, 20, 28, 36, 44, 52, 60), &mut x);
        eighth_round((5, 13, 21, 29, 37, 45, 53, 61), &mut x);
        eighth_round((6, 14, 22, 30, 38, 46, 54, 62), &mut x);
        eighth_round((7, 15, 23, 31, 39, 47, 55, 63), &mut x);
        // Even round (diagonals)
        eighth_round((0, 9, 18, 27, 36, 45, 54, 63), &mut x);
        eighth_round((1, 10, 19, 28, 37, 46, 55, 56), &mut x);
        eighth_round((2, 11, 20, 29, 38, 47, 48, 57), &mut x);
        eighth_round((3, 12, 21, 30, 39, 40, 49, 58), &mut x);
        eighth_round((4, 13, 22, 31, 32, 41, 50, 59), &mut x);
        eighth_round((5, 14, 23, 24, 33, 42, 51, 60), &mut x);
        eighth_round((6, 15, 16, 25, 34, 43, 52, 61), &mut x);
        eighth_round((7, 8, 17, 26, 35, 44, 53, 62), &mut x);
    }
    for (i, elem) in x.iter_mut().enumerate() {
        *elem += input[i]
    }
    x
}
