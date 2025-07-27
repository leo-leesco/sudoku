const SQUARE_SIZE: usize = 3;
const SIZE: usize = SQUARE_SIZE * SQUARE_SIZE;

pub type Ligne = [Option<u8>; SIZE];
pub type Colonne = [Option<u8>; SIZE];
pub type Carre = [[Option<u8>; SQUARE_SIZE]; SQUARE_SIZE];
pub type Sudoku = [Ligne; SIZE];

pub fn ligne(grille: Sudoku, l: usize) -> Ligne {
    grille[l]
}

pub fn colonne(grille: Sudoku, l: usize) -> Colonne {
    let mut col: Colonne = [None; SIZE];
    for i in 0..SIZE {
        col[i] = grille[i][l]
    }
    col
}

pub fn carre(grille: Sudoku, i: usize, j: usize) -> Carre {
    assert!(i < SQUARE_SIZE && j < SQUARE_SIZE);
    let mut sq: Carre = [[None; SQUARE_SIZE]; SQUARE_SIZE];
    for a in 0..SQUARE_SIZE {
        for b in 0..SQUARE_SIZE {
            sq[a][b] = grille[SQUARE_SIZE * i + a][SQUARE_SIZE * j + b]
        }
    }
    sq
}

pub fn valid(grille: Sudoku) -> bool {
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
