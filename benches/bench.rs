#![feature(test)]

extern crate test;

use test::Bencher;


#[bench]
fn million_sum_inline(b: &mut Bencher) {
    b.iter(|| {
        499999500000_u64
    })
}


#[bench]
fn million_range_sum(b: &mut Bencher) {
    b.iter(|| -> u64 {
        (1..100_0000)
            .sum()
    })
}


#[bench]
fn million_range_sum_with_filter(b: &mut Bencher) {
    b.iter(|| -> u64 {
        (1..100_0000)
            .filter(|&x| x > 0)     // always true
            .sum()
    })
}


#[bench]
fn million_range_sum_with_filter2(b: &mut Bencher) {
    b.iter(|| -> u64 {
        (1..100_0000)
            .filter(|&x| x > 0)     // always true
            .filter(|&x| x < 0)     // always false
            .sum()
    })
}


#[bench]
fn million_for_sum(b: &mut Bencher) {
    b.iter(|| -> u64 {
        let mut total = 0;
        for i in 1..100_0000 {
            total += i;
        }
        total
    })
}


#[bench]
fn million_for_sum_with_filter(b: &mut Bencher) {
    b.iter(|| -> u64 {
        let mut total = 0;
        for i in 1..100_0000 {
            if i > 0 {
                total += i;
            }
        }
        total
    })
}


#[bench]
fn million_for_sum_with_filter2(b: &mut Bencher) {
    b.iter(|| -> u64 {
        let mut total = 0;
        for i in 1..100_0000 {
            if i > 0 && i < 0 {
                total += i;
            }
        }
        total
    })
}


#[bench]
fn find_char_in_pattern_match(b: &mut Bencher) {
    let data = test::black_box("this is test data.");
    b.iter(|| {
        let mut result = vec![];
        for c in data.chars() {
            result.push(match c {
                ' '  |
                ','  |
                '.'  |
                '!'  |
                '?'  |
                ';'  |
                '\'' |
                '"'  |
                ':'  |
                '\t' |
                '\n' |
                '('  |
                ')'  |
                '-'  => true,
                _    => false,
            });
        };
        result
    })
}


#[bench]
fn find_char_in_pattern_contains(b: &mut Bencher) {
    let data = test::black_box("this is test data.");
    b.iter(|| {
        let delims = " ,.!?;'\":\t\n()-";
        let mut result = vec![];
        for c in data.chars() {
            result.push(delims.contains(c));
        };
        result
    })
}


#[bench]
fn find_char_in_pattern_for(b: &mut Bencher) {
    let data = test::black_box("this is test data.");
    b.iter(|| {
        let delims = " ,.!?;'\":\t\n()-";
        let delims = [' ', ',', '.', '!', '?', ';', '\'', '"', ':', '\t', '\n', '(', ')', '-'];
        let mut result = vec![];
        for c in data.chars() {
            for &i in delims.iter() {
                if i == c {
                    result.push(true);
                    continue;
                }
            }
            result.push(false);
        };
        result
    })
}
