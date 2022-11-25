use lev_dist::EditDistance;

#[track_caller]
fn check(f: EditDistance, expected: usize, x: &str, y: &str) {
    let actual = f(x, y);
    assert_eq!(actual, expected, "{x:?} {y:?}");
}

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

data_mod!(data_lt64_ascii, lt64_ascii);
data_mod!(data_lt64_unicode, lt64_unicode);
data_mod!(data_eq64_ascii, eq64_ascii);
data_mod!(data_eq64_unicode, eq64_unicode);
data_mod!(data_smoketest, smoketest);
