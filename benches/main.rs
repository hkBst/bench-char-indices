#![feature(test)]

extern crate test;

use bench_char_indices::*;

use std::iter::repeat_n;
const LEN: usize = 10_000;

macro_rules! fn_bench_check_raw {
    ($name:ident, $unit:ty, $check_raw:ident) => {
        fn $name(b: &mut test::Bencher, s: &str, expected: $unit) {
            let input: String = test::black_box(repeat_n(s, LEN).collect());
            assert_eq!(input.len(), LEN * s.len());
            b.iter(|| {
                let mut output = vec![];

                $check_raw(&input, |range, res| output.push((range, res)));
                assert_eq!(output.len(), LEN);
                assert_eq!(output[0], ((0..s.len()), Ok(expected)));
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
    bench_check_raw_str_while(b, "a", 'a');
}

#[bench]
fn bench_check_raw_str_ascii_char_indices(b: &mut test::Bencher) {
    bench_check_raw_str_char_indices(b, "a", 'a');
}
