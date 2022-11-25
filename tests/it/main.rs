use lev_dist::EditDistance;

#[track_caller]
fn check(f: EditDistance, expected: usize, x: &str, y: &str) {
    let actual = f(x, y);
    assert_eq!(actual, expected, "{x:?} {y:?}");
}

#[track_caller]
fn check_data(f: EditDistance, data: &[(usize, &str, &str)]) {
    for (expected, x, y) in data {
        check(f, *expected, x, y);
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

            #[test]
            fn full_matrix() {
                crate::check_data(lev_dist::full_matrix, DATA);
            }

            #[test]
            fn two_rows() {
                crate::check_data(lev_dist::two_rows, DATA);
            }

            #[test]
            fn one_row() {
                crate::check_data(lev_dist::one_row, DATA);
            }

            #[test]
            fn myers() {
                crate::check_data(lev_dist::myers, DATA);
            }
        }
    };
}

data_mod!(data_eq64_ascii, eq64_ascii);
data_mod!(data_eq64_unicode, eq64_unicode);
data_mod!(data_eq_1000_ascii, eq_1000_ascii);
data_mod!(data_eq_100_ascii, eq_100_ascii);
data_mod!(data_eq_1024_ascii, eq_1024_ascii);
data_mod!(data_eq_128_ascii, eq_128_ascii);
data_mod!(data_eq_2000_ascii, eq_2000_ascii);
data_mod!(data_eq_2048_ascii, eq_2048_ascii);
data_mod!(data_eq_256_ascii, eq_256_ascii);
data_mod!(data_eq_500_ascii, eq_500_ascii);
data_mod!(data_lt64_ascii, lt64_ascii);
data_mod!(data_lt64_unicode, lt64_unicode);
data_mod!(data_smoketest, smoketest);
