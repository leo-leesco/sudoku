type Ligne = [Option<u8>; 9];
type Colonne = [Option<u8>; 9];
type Carre = [[Option<u8>; 3]; 3];
type Sudoku = [Ligne; 9];

fn ligne(grille: Sudoku, l: usize) -> Ligne {
    grille[l]
}

fn colonne(grille: Sudoku, l: usize) -> Colonne {
    let mut col: Colonne = [None; 9];
    for i in 0..9 {
        col[i] = grille[i][l]
    }
    col
}

fn carre(grille: Sudoku, i: usize, j: usize) -> Carre {
    assert!(i < 3 && j < 3);
    let mut sq: Carre = [[None; 3]; 3];
    for a in 0..3 {
        for b in 0..3 {
            sq[a][b] = grille[3 * i + a][3 * j + b]
        }
    }
    sq
}

fn valid(grille: Sudoku) -> bool {
    todo!()
}

macro_rules! sudoku {
    ( $( [ $( $num:expr ),* ] ),* $(,)? ) => {
        [
            $(
                [
                    $(
                        match $num {
                            0 => None,
                            n => Some(n),
                        }
                    ),*
                ]
            ),*
        ]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: Sudoku = sudoku![
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

    #[test]
    fn extract_ligne() {
        assert_eq!(result, 4);
    }
}
