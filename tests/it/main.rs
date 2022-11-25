use lev_dist::{full_matrix, naive, one_row, two_rows, EditDistance};

#[track_caller]
fn check(f: EditDistance, expected: usize, x: &str, y: &str) {
    let actual = f(x, y);
    assert_eq!(actual, expected, "{x:?} {y:?}");
}

fn test_smokecheck(ed: EditDistance) {
    check(ed, 4, "toil", "trouble");
    check(ed, 3, "kitten", "sitting");
    check(ed, 2, "book", "back");
    check(ed, 0, "", "");
    check(ed, 1, "", "a");
    check(ed, 1, "a", "");
    check(ed, 0, "lovemuffin", "lovemuffin");
    check(ed, 2, "üö", "uo");
    check(ed, 0, "üö", "üö");
    check(ed, 2, "üö", "üöüö");
    check(ed, 0, "☀☂☃☄", "☀☂☃☄");
    check(ed, 3, "ฎ ฏ ฐ", "a b c");
    check(ed, 1, "Café", "Cafe");
    check(ed, 2, "𐐷𤭢", "$%");
}

#[test]
fn smokecheck_naive() {
    test_smokecheck(naive);
}

#[test]
fn smokecheck_full_matrix() {
    test_smokecheck(full_matrix);
}

#[test]
fn smokecheck_two_rows() {
    test_smokecheck(two_rows);
}

#[test]
fn smokecheck_one_row() {
    test_smokecheck(one_row);
}
