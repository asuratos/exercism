use minesweeper::{annotate, Board};

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

// board tests
fn make_board_3x3() -> Board {
    Board::from_rowlist(&["123", "456", "789"])
}
// neighbor check
#[test]
fn corner_neighbors() {
    let b = make_board_3x3();
    let mut neighbors = b.get_neighbors_of(0);

    // upper left
    neighbors.sort();
    assert_eq!(neighbors, vec![1, 3, 4]);

    //upper right
    neighbors = b.get_neighbors_of(2);
    neighbors.sort();
    assert_eq!(neighbors, vec![1, 4, 5]);

    //lower left
    neighbors = b.get_neighbors_of(6);
    neighbors.sort();
    assert_eq!(neighbors, vec![3, 4, 7]);

    //lower right
    neighbors = b.get_neighbors_of(8);
    neighbors.sort();
    assert_eq!(neighbors, vec![4, 5, 7]);
}

#[test]
fn wall_neighbors() {
    let b = make_board_3x3();
    let mut neighbors;

    // upper wall
    neighbors = b.get_neighbors_of(1);
    neighbors.sort();
    assert_eq!(neighbors, vec![0, 2, 3, 4, 5]);

    // left wall
    neighbors = b.get_neighbors_of(3);
    neighbors.sort();
    assert_eq!(neighbors, vec![0, 1, 4, 6, 7]);

    // right wall
    neighbors = b.get_neighbors_of(5);
    neighbors.sort();
    assert_eq!(neighbors, vec![1, 2, 4, 7, 8]);

    // bottom wall
    neighbors = b.get_neighbors_of(7);
    neighbors.sort();
    assert_eq!(neighbors, vec![3, 4, 5, 6, 8]);
}

#[test]
fn center_neighbors() {
    let b = make_board_3x3();
    let mut neighbors = b.get_neighbors_of(4);
    neighbors.sort();
    assert_eq!(neighbors, vec![0, 1, 2, 3, 5, 6, 7, 8]);
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
#[ignore]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
#[ignore]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
#[ignore]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
#[ignore]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
#[ignore]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
#[ignore]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
#[ignore]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
#[ignore]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
#[ignore]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
#[ignore]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
#[ignore]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
