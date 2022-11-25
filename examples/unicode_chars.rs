use rand::{rngs::ThreadRng, Rng};

fn random_unicode_char(r: &mut ThreadRng) -> char {
    loop {
        let c = r.gen_range(0..=char::MAX as _);
        if let Some(c) = std::char::from_u32(c) {
            if has_rust_nice_str(c) {
                return c;
            }
        }
    }
}

fn has_rust_nice_str(c: char) -> bool {
    let cs = format!("{c:?}");
    let csl = cs.chars().count();
    return csl == 3;
}

fn main() {
    let mut r = rand::thread_rng();
    let mut chars = Vec::new();
    for _ in 0..300 {
        chars.push(random_unicode_char(&mut r));
    }
    println!("{:?}", chars);
}
