use std::cmp::min;

// https://github.com/ka-weihe/fast-levenshtein/blob/main/levenshtein.go

pub struct Myers {
    peq: Box<[u64]>,
}

const CMAX: usize = char::MAX as usize;

impl Myers {
    pub fn new() -> Self {
        Self {
            peq: vec![0u64; CMAX].into_boxed_slice(),
        }
    }

    pub fn dist(&mut self, a: &str, b: &str) -> usize {
        let a_len = a.chars().count();
        let b_len = b.chars().count();
        if a_len < b_len {
            return self.dist(b, a);
        }
        if b_len == 0 {
            return a_len;
        }
        if a_len <= 64 {
            return self.m64(a, b);
        }
        return self.mx(a, b);
    }

    pub(super) fn m64(&mut self, a: &str, b: &str) -> usize {
        let mut pv = u64::MAX;
        let mut mv = 0;
        let mut sc = 0;

        for c in a.chars() {
            let c = c as usize;
            self.peq[c] |= 1 << sc;
            sc += 1;
        }

        let ls = 1 << (sc - 1);

        for c in b.chars() {
            let c = c as usize;
            let mut eq = self.peq[c];
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

        for c in a.chars() {
            self.peq[c as usize] = 0;
        }

        sc
    }

    pub(super) fn mx(&mut self, a: &str, b: &str) -> usize {
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

impl Default for Myers {
    fn default() -> Self {
        Self::new()
    }
}
