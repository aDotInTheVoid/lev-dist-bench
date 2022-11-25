#[track_caller]
fn check(f: &mut impl EditDistance, expected: usize, x: &str, y: &str) {
    let actual = f.dist(x, y);
    assert_eq!(actual, expected, "{x:?} {y:?}");
}

#[track_caller]
fn check_data<E: EditDistance>(data: &[(usize, &str, &str)]) {
    let mut f = E::default();
    for (expected, x, y) in data {
        check(&mut f, *expected, x, y);
    }
}

trait EditDistance: Default {
    fn dist(&mut self, a: &str, b: &str) -> usize;
}

#[derive(Default)]
struct OneRow;

impl EditDistance for OneRow {
    fn dist(&mut self, a: &str, b: &str) -> usize {
        lev_dist::one_row(a, b)
    }
}

#[derive(Default)]
struct Myers(lev_dist::Myers);

impl EditDistance for Myers {
    fn dist(&mut self, a: &str, b: &str) -> usize {
        self.0.dist(a, b)
    }
}

macro_rules! data_mod {
    ($mod_name:ident, $name:ident) => {
        mod $mod_name;

        mod $name {
            use crate::$mod_name::DATA;

            // #[test]
            // fn naive() {
            //     crate::check_data(lev_dist::naive, DATA);
            // }

            // #[test]
            // fn full_matrix() {
            //     crate::check_data(lev_dist::full_matrix, DATA);
            // }

            // #[test]
            // fn two_rows() {
            //     crate::check_data(lev_dist::two_rows, DATA);
            // }

            #[test]
            fn one_row() {
                crate::check_data::<crate::OneRow>(DATA);
            }

            #[test]
            fn myers() {
                crate::check_data::<crate::Myers>(DATA);
            }
        }
    };
}

data_mod!(data_16, l16);
data_mod!(data_32, l32);
data_mod!(data_64, l64);
data_mod!(data_128, l128);
data_mod!(data_256, l256);
data_mod!(data_512, l512);
data_mod!(data_1024, l1024);
data_mod!(data_2048, l2048);
