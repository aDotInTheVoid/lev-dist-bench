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

pub fn full_matrix(s: &str, t: &str) -> usize {
    let m = s.chars().count();
    let n = t.chars().count();

    let mut d = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        d[i][0] = i;
    }
    for j in 1..=n {
        d[0][j] = j;
    }

    for (j_, tj) in t.chars().enumerate() {
        let j = j_ + 1;
        for (i_, ti) in s.chars().enumerate() {
            let i = i_ + 1;

            let sub = if ti == tj { 0 } else { 1 };

            let del_cost = d[i - 1][j] + 1;
            let ins_cost = d[i][j - 1] + 1;
            let sub_cost = d[i - 1][j - 1] + sub;

            d[i][j] = min(del_cost, ins_cost, sub_cost);
        }
    }

    return d[m][n];
}

pub fn two_rows(s: &str, t: &str) -> usize {
    let n = t.chars().count();

    let mut v0 = vec![0; n + 1];
    let mut v1 = vec![0; n + 1];

    for i in 0..=n {
        v0[i] = i;
    }

    for (i, si) in s.chars().enumerate() {
        v1[0] = i + 1;

        for (j, tj) in t.chars().enumerate() {
            let sub = if si == tj { 0 } else { 1 };

            let del_cost = v0[j + 1] + 1;
            let ins_cost = v1[j] + 1;
            let sub_cost = v0[j] + sub;

            v1[j + 1] = min(del_cost, ins_cost, sub_cost);
        }

        std::mem::swap(&mut v0, &mut v1);
    }

    return v0[n];
}
