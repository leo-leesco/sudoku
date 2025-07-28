const SQUARE_SIZE: usize = 3;
pub const SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

pub type Ligne<T> = [T; SIZE];
pub type Colonne<T> = [T; SIZE];
pub type Carre<T> = [[T; SQUARE_SIZE]; SQUARE_SIZE];
pub type Sudoku<T> = [Ligne<T>; SIZE];

pub fn ligne<T: Copy>(grille: Sudoku<T>, l: usize) -> Ligne<T> {
    grille[l]
}

pub fn colonne<T: Copy + Default>(grille: Sudoku<T>, l: usize) -> Colonne<T> {
    let mut col: Colonne<T> = [T::default(); SIZE];
    for i in 0..SIZE {
        col[i] = grille[i][l]
    }
    col
}

pub fn carre<T: Copy + Default>(grille: Sudoku<T>, i: usize, j: usize) -> Carre<T> {
    assert!(i < SQUARE_SIZE && j < SQUARE_SIZE);
    let mut sq: Carre<T> = [[T::default(); SQUARE_SIZE]; SQUARE_SIZE];
    for a in 0..SQUARE_SIZE {
        for b in 0..SQUARE_SIZE {
            sq[a][b] = grille[SQUARE_SIZE * i + a][SQUARE_SIZE * j + b]
        }
    }
    sq
}

pub fn valid(grille: Sudoku<Option<u8>>) -> bool {
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

pub fn barre(grille: Sudoku<Option<u8>>, chiffre: u8) -> Sudoku<bool> {
    let mut mask: Sudoku<bool> =
        grille.map(|ligne| ligne.map(|v| if let Some(_) = v { false } else { true }));
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

pub fn one_step_trivial(mut grille: Sudoku<Option<u8>>) -> Result<Sudoku<Option<u8>>, ()> {
    let mut mask: Sudoku<bool>;
    let mut updated = false;
    for chiffre in 1..=SIZE {
        mask = barre(grille, chiffre as u8);

        for i in 0..SIZE {
            if let Some(j) = only_possible_slot(ligne(mask, i)) {
                grille[i][j] = Some(chiffre as u8);
                updated = true;

                if cfg!(test) {
                    println!("added {chiffre} at ({i},{j})")
                }
            }

            if let Some(j) = only_possible_slot(colonne(mask, i)) {
                grille[j][i] = Some(chiffre as u8);
                updated = true;

                if cfg!(test) {
                    println!("added {chiffre} at ({i},{j})")
                }
            }

            if let Some(j) = only_possible_slot(
                carre(mask, i / SQUARE_SIZE, i % SQUARE_SIZE) // on lit de gauche à droite puis de haut en bas
                    .iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ) {
                grille[j / SQUARE_SIZE][j % SQUARE_SIZE] = Some(chiffre as u8);
                updated = true;

                if cfg!(test) {
                    println!("added {chiffre} in square {i} in position {j}")
                }
            }
        }
    }
    if updated { Ok(grille) } else { Err(()) }
}
