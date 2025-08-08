const SQUARE_SIZE: usize = 3;
const SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

type Ligne<T> = [T; SIZE];
type Colonne<T> = [T; SIZE];
type Carre<T> = [[T; SQUARE_SIZE]; SQUARE_SIZE];
type Sudoku<T> = [Ligne<T>; SIZE];

fn ligne<T: Copy>(grille: Sudoku<T>, l: usize) -> Ligne<T> {
    grille[l]
}

fn colonne<T: Copy + Default>(grille: Sudoku<T>, l: usize) -> Colonne<T> {
    let mut col: Colonne<T> = [T::default(); SIZE];
    for i in 0..SIZE {
        col[i] = grille[i][l]
    }
    col
}

fn carre<T: Copy + Default>(grille: Sudoku<T>, i: usize, j: usize) -> Carre<T> {
    assert!(i < SQUARE_SIZE && j < SQUARE_SIZE);
    let mut sq: Carre<T> = [[T::default(); SQUARE_SIZE]; SQUARE_SIZE];
    for a in 0..SQUARE_SIZE {
        for b in 0..SQUARE_SIZE {
            sq[a][b] = grille[SQUARE_SIZE * i + a][SQUARE_SIZE * j + b]
        }
    }
    sq
}

fn valid(grille: Sudoku<Option<u8>>) -> bool {
    let mut numbers: Vec<u8>;

    for i in 0..SIZE {
        numbers = ligne(grille, i).iter().filter_map(|v| *v).collect();
        numbers.sort();
        if numbers.windows(2).any(|w| w[0] == w[1]) {
            return false;
        }
    }

    for j in 0..SIZE {
        numbers = colonne(grille, j).iter().filter_map(|v| *v).collect();
        numbers.sort();
        if numbers.windows(2).any(|w| w[0] == w[1]) {
            return false;
        }
    }

    for i in 0..SQUARE_SIZE {
        for j in 0..SQUARE_SIZE {
            numbers = carre(grille, i, j)
                .iter()
                .flatten()
                .filter_map(|v| *v)
                .collect();
            numbers.sort();
            if numbers.windows(2).any(|w| w[0] == w[1]) {
                return false;
            }
        }
    }

    true
}

#[macro_export]
macro_rules! sudoku {
    ( $( [ $( $num:expr ),* ] ),* $(,)? ) => {
        [
            $(
                [
                    $(
                        if $num == 0 { None } else { Some($num) }
                    ),*
                ]
            ),*
        ]
    };
}

#[macro_export]
macro_rules! ligne {
    ( $($num:expr),* $(,)? ) => {
        [
            $(
                if $num == 0 { None } else { Some($num) }
            ),*
        ]
    };
}

fn barre_ligne(mut grille: Sudoku<bool>, ligne: usize) -> Sudoku<bool> {
    for j in 0..SIZE {
        grille[ligne][j] = false
    }
    grille
}

fn barre_colonne(mut grille: Sudoku<bool>, colonne: usize) -> Sudoku<bool> {
    for i in 0..SIZE {
        grille[i][colonne] = false
    }
    grille
}

fn barre_carre(mut grille: Sudoku<bool>, i: usize, j: usize) -> Sudoku<bool> {
    assert!(i < SQUARE_SIZE && j < SQUARE_SIZE);
    for a in 0..SQUARE_SIZE {
        for b in 0..SQUARE_SIZE {
            grille[3 * i + a][3 * j + b] = false
        }
    }
    grille
}

fn barre(grille: Sudoku<Option<u8>>, chiffre: u8) -> Sudoku<bool> {
    let mut mask: Sudoku<bool> = grille.map(|ligne| ligne.map(|v| v.is_none())); // barre
    // les chiffres déjà placés

    // on barre les positions impossibles en plus
    for i in 0..SIZE {
        for j in 0..SIZE {
            if let Some(n) = grille[i][j] {
                if n == chiffre {
                    mask = barre_ligne(mask, i);
                    mask = barre_colonne(mask, j);
                    mask = barre_carre(mask, i / 3, j / 3);
                }
            }
        }
    }
    mask
}

fn only_possible_slot(ligne: Ligne<bool>) -> Option<usize> {
    let mut index = None;
    for (i, &b) in ligne.iter().enumerate() {
        if b {
            if index.is_some() {
                return None; // More than one true found
            }
            index = Some(i);
        }
    }
    index // Either None (no true), or Some(index) if exactly one
}

#[derive(Debug, PartialEq, Eq)]
pub struct UpdatedGrid<T> {
    grid: Sudoku<T>,
    updates: usize,
}

/// returns the updated grid and the number of new numbers added during this pass
fn trivial_digit(mut grille: Sudoku<Option<u8>>, chiffre: u8) -> UpdatedGrid<Option<u8>> {
    let mut mask = barre(grille, chiffre);
    let mut nb_updates = 0;

    for i in 0..SIZE {
        if let Some(j) = only_possible_slot(ligne(mask, i)) {
            if grille[i][j].is_none() {
                nb_updates += 1;
                mask = barre_ligne(mask, i);
                mask = barre_colonne(mask, j);
                mask = barre_carre(mask, i / 3, j / 3);
                grille[i][j] = Some(chiffre);

                eprintln!("found {chiffre} in line at ({i},{j})")
            }
        }
    }

    for j in 0..SIZE {
        if let Some(i) = only_possible_slot(colonne(mask, j)) {
            if grille[i][j].is_none() {
                nb_updates += 1;
                mask = barre_ligne(mask, i);
                mask = barre_colonne(mask, j);
                mask = barre_carre(mask, i / 3, j / 3);
                grille[i][j] = Some(chiffre);

                eprintln!("found {chiffre} in column at ({i},{j})")
            }
        }
    }

    for i in 0..SQUARE_SIZE {
        for j in 0..SQUARE_SIZE {
            if let Some(k) = only_possible_slot(
                carre(mask, i, j)
                    .iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ) {
                let (a, b) = (k / SQUARE_SIZE, k % SQUARE_SIZE);
                let ip = SQUARE_SIZE * i + a;
                let jp = SQUARE_SIZE * j + b;
                if grille[ip][jp].is_none() {
                    nb_updates += 1;
                    mask = barre_ligne(mask, ip);
                    mask = barre_colonne(mask, jp);
                    mask = barre_carre(mask, i, j);
                    grille[ip][jp] = Some(chiffre);

                    eprintln!("found {chiffre} in square at ({ip},{jp})")
                }
            }
        }
    }

    UpdatedGrid {
        grid: grille,
        updates: nb_updates,
    }
}

fn trivial(mut grille: Sudoku<Option<u8>>) -> UpdatedGrid<Option<u8>> {
    let mut nb_updates = 0;
    let mut added;
    for chiffre in 1..=SIZE {
        UpdatedGrid {
            grid: grille,
            updates: added,
        } = trivial_digit(grille, chiffre as u8);
        nb_updates += added;
    }

    UpdatedGrid {
        grid: grille,
        updates: nb_updates,
    }
}

pub enum StopReason {
    Invalid,
    NoProgressMade,
    Full,
}

pub fn solve(grille: Sudoku<Option<u8>>) -> Result<UpdatedGrid<Option<u8>>, StopReason> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_sudoku(sudoku: Sudoku<Option<u8>>) -> Sudoku<u8> {
        sudoku.map(|ligne| {
            ligne.map(|v| {
                if let Some(num) = v {
                    assert!(num <= SIZE as u8);
                    num
                } else {
                    0
                }
            })
        })
    }

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
    }

    #[test]
    fn test_trivial_digit() {
        assert_eq!(
            trivial_digit(MISS_ONE, 5),
            UpdatedGrid {
                grid: VALID,
                updates: 1
            }
        );

        assert!(valid(trivial_digit(WITH_HOLES, 2).grid));
        assert_eq!(
            trivial_digit(WITH_HOLES, 2),
            UpdatedGrid {
                grid: sudoku![
                    [5, 0, 0, 0, 0, 2, 1, 0, 4],
                    [8, 2, 0, 4, 0, 0, 0, 3, 0],
                    [3, 6, 0, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 8, 3, 6, 0, 0],
                    [0, 8, 0, 0, 0, 0, 0, 9, 0],
                    [0, 0, 2, 9, 1, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0, 0, 2, 6],
                    [2, 3, 0, 0, 0, 4, 0, 0, 7],
                    [9, 0, 6, 1, 2, 0, 0, 0, 8]
                ],
                updates: 3
            },
            "got instead {:?}",
            from_sudoku(trivial_digit(WITH_HOLES, 2).grid)
        )
    }

    #[test]
    fn test_trivial() {
        assert_eq!(
            trivial(MISS_ONE),
            UpdatedGrid {
                grid: VALID,
                updates: 1
            }
        );

        let UpdatedGrid {
            grid: with_holes_partial,
            updates,
        } = trivial(WITH_HOLES);
        assert!(valid(with_holes_partial));
        assert_eq!(
            (with_holes_partial, updates),
            (
                sudoku![
                    [5, 0, 0, 8, 0, 2, 1, 6, 4],
                    [8, 2, 0, 4, 0, 0, 0, 3, 0],
                    [3, 6, 4, 0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 8, 3, 6, 0, 0],
                    [0, 8, 3, 0, 4, 0, 0, 9, 0],
                    [0, 0, 2, 9, 1, 0, 0, 0, 3],
                    [0, 0, 0, 0, 0, 8, 0, 2, 6],
                    [2, 3, 8, 0, 0, 4, 0, 1, 7],
                    [9, 0, 6, 1, 2, 0, 3, 0, 8]
                ],
                13
            ),
            "got instead {:?}",
            from_sudoku(trivial(WITH_HOLES).grid)
        );
    }
}
