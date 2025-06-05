#![feature(test)]

extern crate test;

use bench_char_indices::*;

use std::iter::repeat_n;
const LEN: usize = 10_000;

macro_rules! fn_bench_check_raw {
    ($name:ident, $unit:ty, $check_raw:ident) => {
        fn $name(b: &mut test::Bencher, s: &str, expected: Vec<$unit>) {
            let input: String = test::black_box(repeat_n(s, LEN).collect());
            assert_eq!(input.len(), LEN * s.len());
            b.iter(|| {
                let mut output = vec![];

                $check_raw(&input, |range, res| output.push((range, res)));
                assert_eq!(output.len(), LEN * s.chars().count());
                for ((i, &e), (p, c)) in expected.iter().enumerate().zip(s.char_indices()) {
                    assert_eq!(output[i], ((p..p + c.len_utf8()), Ok(e)));
                }
            });
        }
    };
}

fn_bench_check_raw!(bench_check_raw_str_while, char, check_raw_str_while);
fn_bench_check_raw!(
    bench_check_raw_str_char_indices,
    char,
    check_raw_str_char_indices
);

#[bench]
fn bench_check_raw_str_ascii_while(b: &mut test::Bencher) {
    bench_check_raw_str_while(b, "a", repeat_n('a', LEN).collect());
}

#[bench]
fn bench_check_raw_str_ascii_char_indices(b: &mut test::Bencher) {
    bench_check_raw_str_char_indices(b, "a", repeat_n('a', LEN).collect());
}

#[bench]
fn bench_check_raw_str_unicode_while(b: &mut test::Bencher) {
    bench_check_raw_str_while(b, "🦀", repeat_n('🦀', LEN).collect());
}

#[bench]
fn bench_check_raw_str_unicode_char_indices(b: &mut test::Bencher) {
    bench_check_raw_str_char_indices(b, "🦀", repeat_n('🦀', LEN).collect());
}

#[bench]
fn bench_check_raw_str_mixed_while(b: &mut test::Bencher) {
    bench_check_raw_str_while(
        b,
        "a🦀",
        (0..2 * LEN)
            .map(|i| if i % 2 == 0 { 'a' } else { '🦀' })
            .collect(),
    );
}

#[bench]
fn bench_check_raw_str_mixed_char_indices(b: &mut test::Bencher) {
    bench_check_raw_str_char_indices(
        b,
        "a🦀",
        (0..2 * LEN)
            .map(|i| if i % 2 == 0 { 'a' } else { '🦀' })
            .collect(),
    );
}
