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

pub fn one_row(s: &str, t: &str) -> usize {
    let n = t.chars().count();

    let mut cur = vec![0; n + 1];

    for i in 1..=n {
        cur[i] = i;
    }

    for (i, si) in s.chars().enumerate() {
        let mut pre = cur[0];
        cur[0] = i + 1;
        for (j, tj) in t.chars().enumerate() {
            let sub = if si == tj { 0 } else { 1 };

            let mut tmp = cur[j + 1];

            let del_cost = tmp + 1;
            let ins_cost = cur[j] + 1;
            let sub_cost = pre + sub;

            tmp = cur[j + 1];
            cur[j + 1] = min(del_cost, ins_cost, sub_cost);
            pre = tmp;
        }
    }
    cur[n]
}

pub fn myers(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();
    if a_len < b_len {
        return myers(b, a);
    }
    if b_len == 0 {
        return a_len;
    }
    if a_len <= 64 {
        return myers::m64(a, b);
    }
    return myers::mx(a, b);
}

mod myers {
    use std::cmp::min;

    const CMAX: usize = char::MAX as usize;

    pub(super) fn m64(a: &str, b: &str) -> usize {
        let mut peq: Box<[u64]> = vec![0u64; CMAX].into_boxed_slice();

        let mut pv = u64::MAX;
        let mut mv = 0;
        let mut sc = 0;

        for c in a.chars() {
            let c = c as usize;
            peq[c] |= 1 << sc;
            sc += 1;
        }

        let ls = 1 << (sc - 1);

        for c in b.chars() {
            let c = c as usize;
            let mut eq = peq[c];
            let xv = eq | mv;
            eq |= (eq & pv).wrapping_add(pv) ^ pv;
            mv |= !(eq | pv);
            pv &= eq;

            if (mv & ls) != 0 {
                sc += 1;
            }
            if (pv & ls) != 0 {
                sc -= 1;
            }
            mv = (mv << 1) | 1;
            pv = (pv << 1) | !(xv | mv);
            mv &= xv;
        }

        sc
    }

    pub(super) fn mx(a: &str, b: &str) -> usize {
        let mut peq: Box<[u64]> = vec![0u64; CMAX].into_boxed_slice();

        let s1 = a.chars().collect::<Vec<_>>();
        let s2 = b.chars().collect::<Vec<_>>();

        let n = s1.len();
        let m = s2.len();
        let hsize = 1 + ((n - 1) / 64);
        let vsize = 1 + ((m - 1) / 64);

        let mut phc = vec![u64::MAX; hsize];
        let mut mhc = vec![0; hsize];

        let mut j = 0;

        while j < vsize - 1 {
            let mut mv = 0;
            let mut pv = u64::MAX;
            let start = j * 64;

            let vlen = min(64, m) + start;

            for k in start..vlen {
                peq[s2[k] as usize] |= 1 << (k & 63);
            }

            for i in 0..n {
                let eq = peq[s1[i] as usize];
                let pb = (phc[i / 64] >> (i & 63)) & 1;
                let mb = (mhc[i / 64] >> (i & 63)) & 1;
                let xv = eq | mv;
                let xh = ((((eq | mb) & pv).wrapping_add(pv)) ^ pv) | eq | mb;
                let mut ph = mv | !(xh | pv);
                let mut mh = pv & xh;
                if ((ph >> 63) ^ pb) != 0 {
                    phc[i / 64] ^= 1 << (i & 63);
                    // println!("Flipped phc {i}");
                }
                if ((mh >> 63) ^ mb) != 0 {
                    mhc[i / 64] ^= 1 << (i & 63);
                    // println!("Flipped mhc {i}");
                }
                ph = (ph << 1) | pb;
                mh = (mh << 1) | mb;
                pv = mh | !(xv | ph);
                mv = ph & xv;
                // println!("{eq} {pb} {mb} {xv} {xh} {ph} {mh}");
            }

            for k in start..vlen {
                peq[s2[k] as usize] = 0;
            }

            j += 1;
        }

        let mut mv = 0;
        let mut pv = u64::MAX;
        let start = j * 64;
        let vlen = min(64, m - start) + start;

        for k in start..vlen {
            peq[s2[k] as usize] |= 1 << (k & 63);
        }
        let mut sc = m;
        for i in 0..n {
            let eq = peq[s1[i] as usize];
            let pb = (phc[i / 64] >> (i & 63)) & 1;
            let mb = (mhc[i / 64] >> (i & 63)) & 1;
            let xv = eq | mv;
            let xh = ((((eq | mb) & pv).wrapping_add(pv)) ^ pv) | eq | mb;
            let mut ph = mv | !(xh | pv);
            let mut mh = pv & xh;
            sc += ((ph >> ((m - 1) & 63)) & 1) as usize;
            sc -= ((mh >> ((m - 1) & 63)) & 1) as usize;
            if ((ph >> 63) ^ pb) != 0 {
                phc[i / 64] ^= 1 << (i & 63);
            }
            if ((mh >> 63) ^ mb) != 0 {
                mhc[i / 64] ^= 1 << (i & 63);
            }
            ph = (ph << 1) | pb;
            mh = (mh << 1) | mb;
            pv = mh | !(xv | ph);
            mv = ph & xv;
        }
        return sc;
    }
}
