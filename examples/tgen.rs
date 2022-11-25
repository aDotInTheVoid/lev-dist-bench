use std::cmp::max;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use rand::{rngs::ThreadRng, Rng};

fn main() {
    let mut r = rand::thread_rng();

    for l in [2, 16, 32, 64, 128, 256, 512, 1024, 2048] {
        write_test(l, &mut r);
    }
}

fn write_test(l: usize, r: &mut ThreadRng) {
    let name = format!("tests/it/data_{l}.rs");

    let mut out = BufWriter::new(File::create(&name).unwrap());
    writeln!(out, "pub const DATA: &[(usize, &str, &str)] = &[").unwrap();

    for _ in 1..1000 {
        let l1 = l + r.gen_range(0..max(l / 10, 5));
        let l2 = l + r.gen_range(0..max(l / 10, 5));

        let s = gen_string(l1, r);
        let t = gen_string(l2, r);

        let d = edit_distance::edit_distance(&s, &t);

        writeln!(out, "({d}, {s:?}, {t:?}),").unwrap();
    }

    writeln!(out, "];").unwrap();

    println!("wrote {}", name);
}

const CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', '東', '京', 'ß', 'ℝ', '💣', 'Є', 'ق', 'ܔ', 'દ', 'ଖ', '௵',
    '౸',
];

fn gen_string(l1: usize, r: &mut ThreadRng) -> String {
    let mut s = String::with_capacity(l1 * 2);
    for _ in 0..l1 {
        let c = CHARS[r.gen_range(0..CHARS.len())];
        s.push(c);
    }
    s
}
