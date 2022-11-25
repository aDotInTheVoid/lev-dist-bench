pub type EditDistance = fn(&str, &str) -> usize;

fn min(x: usize, y: usize, z: usize) -> usize {
    std::cmp::min(x, std::cmp::min(y, z))
}

// https://en.wikipedia.org/wiki/Levenshtein_distance#Recursive
pub fn naive(x: &str, y: &str) -> usize {
    let Some(x1) = x.chars().next() else { return y.chars().count() };
    let Some(y1) = y.chars().next() else { return x.chars().count() };

    let x_rest = &x[x1.len_utf8()..];
    let y_rest = &y[y1.len_utf8()..];

    dbg!((x1, y1, x_rest, y_rest, x1 == y1));

    if x1 == y1 {
        dbg!(naive(x_rest, y_rest))
    } else {
        let d_insert = naive(x, y_rest);
        let d_delete = naive(x_rest, y);
        let d_swap = naive(x_rest, y_rest);
        1 + min(d_insert, d_delete, d_swap)
    }
}
