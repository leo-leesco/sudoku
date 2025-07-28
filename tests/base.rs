use sudoku::*;

const VALID: Sudoku<Option<u8>> = sudoku![
    [5, 9, 7, 8, 3, 2, 1, 6, 4],
    [8, 2, 1, 4, 6, 9, 7, 3, 5],
    [3, 6, 4, 5, 7, 1, 2, 8, 9],
    [4, 5, 9, 2, 8, 3, 6, 7, 1],
    [1, 8, 3, 7, 4, 6, 5, 9, 2],
    [6, 7, 2, 9, 1, 5, 8, 4, 3],
    [7, 1, 5, 3, 9, 8, 4, 2, 6],
    [2, 3, 8, 6, 5, 4, 9, 1, 7],
    [9, 4, 6, 1, 2, 7, 3, 5, 8]
];

const MISS_ONE: Sudoku<Option<u8>> = sudoku![
    [0, 9, 7, 8, 3, 2, 1, 6, 4],
    [8, 2, 1, 4, 6, 9, 7, 3, 5],
    [3, 6, 4, 5, 7, 1, 2, 8, 9],
    [4, 5, 9, 2, 8, 3, 6, 7, 1],
    [1, 8, 3, 7, 4, 6, 5, 9, 2],
    [6, 7, 2, 9, 1, 5, 8, 4, 3],
    [7, 1, 5, 3, 9, 8, 4, 2, 6],
    [2, 3, 8, 6, 5, 4, 9, 1, 7],
    [9, 4, 6, 1, 2, 7, 3, 5, 8]
];

const WITH_HOLES: Sudoku<Option<u8>> = sudoku![
    [5, 0, 0, 0, 0, 2, 1, 0, 4],
    [8, 0, 0, 4, 0, 0, 0, 3, 0],
    [3, 6, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 8, 3, 6, 0, 0],
    [0, 8, 0, 0, 0, 0, 0, 9, 0],
    [0, 0, 2, 9, 1, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 2, 6],
    [0, 3, 0, 0, 0, 4, 0, 0, 7],
    [9, 0, 6, 1, 0, 0, 0, 0, 8]
];

const DEFAULT: Sudoku<Option<u8>> = sudoku![
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [2, 3, 4, 5, 6, 7, 8, 9, 1],
    [3, 4, 5, 6, 7, 8, 9, 1, 2],
    [4, 5, 6, 7, 8, 9, 1, 2, 3],
    [5, 6, 7, 8, 9, 1, 2, 3, 4],
    [6, 7, 8, 9, 1, 2, 3, 4, 5],
    [7, 8, 9, 1, 2, 3, 4, 5, 6],
    [8, 9, 1, 2, 3, 4, 5, 6, 7],
    [9, 1, 2, 3, 4, 5, 6, 7, 8],
];

#[test]
fn extract_ligne() {
    assert_eq!(ligne(VALID, 0), ligne![5, 9, 7, 8, 3, 2, 1, 6, 4]);
    assert_eq!(
        ligne(WITH_HOLES, 0),
        [
            Some(5),
            None,
            None,
            None,
            None,
            Some(2),
            Some(1),
            None,
            Some(4)
        ]
    )
}

#[test]
fn extract_colonne() {
    assert_eq!(colonne(VALID, 0), ligne![5, 8, 3, 4, 1, 6, 7, 2, 9]);
}

#[test]
fn extract_carre() {
    assert_eq!(carre(VALID, 0, 0), sudoku![[5, 9, 7], [8, 2, 1], [3, 6, 4]]);
}

#[test]
fn assert_valid() {
    assert!(valid(VALID));
    assert!(valid(WITH_HOLES));
}

#[test]
fn assert_invalid() {
    assert!(!valid(DEFAULT))
}

#[test]
fn test_barre() {
    assert_eq!(barre(VALID, 3), [[false; SIZE]; SIZE]);
    assert_eq!(
        barre(WITH_HOLES, 2),
        [
            [
                false, false, false, false, false, false, false, false, false
            ],
            [false, true, false, false, false, false, true, false, true],
            [false, false, false, false, false, false, true, false, true],
            [false, false, false, true, false, false, false, false, true],
            [false, false, false, true, true, false, true, false, true],
            [
                false, false, false, false, false, false, false, false, false
            ],
            [
                false, false, false, false, false, false, false, false, false
            ],
            [true, false, false, true, true, false, false, false, false],
            [false, true, false, false, true, false, false, false, false]
        ]
    )
    // println!("{:?}", barre(WITH_HOLES, 2)) // to get the output via `cargo test -- --nocapture`
}

#[test]
fn test_easy_one_step_trivial() {
    assert_eq!(one_step_trivial(MISS_ONE), Ok(VALID))
}

#[test]
fn test_one_step_trivial() {
    assert_eq!(
        one_step_trivial(WITH_HOLES),
        Ok([
            [
                Some(5),
                None,
                None,
                None,
                None,
                Some(2),
                Some(1),
                Some(6),
                Some(4)
            ],
            [
                Some(8),
                Some(2),
                None,
                Some(4),
                None,
                None,
                None,
                Some(3),
                None
            ],
            [
                Some(3),
                Some(6),
                Some(4),
                None,
                None,
                None,
                None,
                None,
                None
            ],
            [
                None,
                None,
                None,
                None,
                Some(8),
                Some(3),
                Some(6),
                None,
                None
            ],
            [
                None,
                Some(8),
                Some(3),
                None,
                Some(4),
                None,
                None,
                Some(9),
                None
            ],
            [
                None,
                None,
                Some(2),
                Some(9),
                Some(1),
                None,
                None,
                None,
                None
            ],
            [None, None, None, None, None, None, None, Some(2), Some(6)],
            [
                None,
                Some(3),
                None,
                None,
                None,
                Some(4),
                None,
                None,
                Some(7)
            ],
            [
                Some(9),
                None,
                Some(6),
                Some(1),
                None,
                None,
                None,
                None,
                Some(8)
            ]
        ])
    );

    // println!("{:?}", one_step_trivial(WITH_HOLES))
}
