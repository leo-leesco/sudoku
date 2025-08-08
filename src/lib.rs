const SQUARE_SIZE: usize = 3;
pub(crate) const SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

pub(crate) type Ligne<T> = [T; SIZE];
pub(crate) type Colonne<T> = [T; SIZE];
pub(crate) type Carre<T> = [[T; SQUARE_SIZE]; SQUARE_SIZE];
pub(crate) type Sudoku<T> = [Ligne<T>; SIZE];

pub(crate) fn ligne<T: Copy>(grille: Sudoku<T>, l: usize) -> Ligne<T> {
    grille[l]
}

pub(crate) fn colonne<T: Copy + Default>(grille: Sudoku<T>, l: usize) -> Colonne<T> {
    let mut col: Colonne<T> = [T::default(); SIZE];
    for i in 0..SIZE {
        col[i] = grille[i][l]
    }
    col
}

pub(crate) fn carre<T: Copy + Default>(grille: Sudoku<T>, i: usize, j: usize) -> Carre<T> {
    assert!(i < SQUARE_SIZE && j < SQUARE_SIZE);
    let mut sq: Carre<T> = [[T::default(); SQUARE_SIZE]; SQUARE_SIZE];
    for a in 0..SQUARE_SIZE {
        for b in 0..SQUARE_SIZE {
            sq[a][b] = grille[SQUARE_SIZE * i + a][SQUARE_SIZE * j + b]
        }
    }
    sq
}

pub(crate) fn valid(grille: Sudoku<Option<u8>>) -> bool {
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

pub(crate) fn barre(grille: Sudoku<Option<u8>>, chiffre: u8) -> Sudoku<bool> {
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

/// returns the updated grid and the number of new numbers added during this pass
pub(crate) fn trivial_digit(
    mut grille: Sudoku<Option<u8>>,
    chiffre: u8,
) -> (Sudoku<Option<u8>>, usize) {
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

    (grille, nb_updates)
}

pub(crate) fn trivial(mut grille: Sudoku<Option<u8>>) -> (Sudoku<Option<u8>>, usize) {
    let mut nb_updates = 0;
    let mut added;
    for chiffre in 1..=SIZE {
        (grille, added) = trivial_digit(grille, chiffre as u8);
        nb_updates += added;
    }

    (grille, nb_updates)
}
