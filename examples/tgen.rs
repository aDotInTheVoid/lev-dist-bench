use std::fmt::Write;
use std::{cmp::min, io};

use rand::{rngs::ThreadRng, Rng};

fn main() {
    let mut out = String::new();

    let mut rng = rand::thread_rng();

    for i in 1..1000 {
        // let ls = rng.gen_range(0..=min(64, i));
        // let lt = rng.gen_range(0..=min(64, i));

        let ls = 64;
        let lt = 64;

        // let s = random_string(ls, &mut rng);
        // let t = random_string(lt, &mut rng);

        let s = random_unicode_string(ls, &mut rng);
        let t = random_unicode_string(lt, &mut rng);

        let d = edit_distance::edit_distance(&s, &t);

        writeln!(out, "({d}, {s:?}, {t:?}),").unwrap();
    }

    println!("{}", out);
}

fn random_string(n: usize, r: &mut ThreadRng) -> String {
    let mut s = String::with_capacity(n);
    for _ in 0..n {
        let c = r.gen_range(0..=26) as u8 + b'a';
        s.push(c as char);
    }
    s
}

fn random_unicode_string(n: usize, r: &mut ThreadRng) -> String {
    let mut s = String::with_capacity(n);
    for _ in 0..n {
        let c = random_unicode_char(r);

        s.push(c);
    }
    s
}

fn random_unicode_char(r: &mut ThreadRng) -> char {
    loop {
        let c = r.gen_range(0..=char::MAX as _);
        if let Some(c) = std::char::from_u32(c) {
            return c;
        }
    }
}
