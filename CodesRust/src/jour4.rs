//code opti


const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // droite
    (0, -1),  // gauche
    (1, 0),   // bas
    (-1, 0),  // haut
    (1, 1),   // bas-droite
    (1, -1),  // bas-gauche
    (-1, 1),  // haut-droite
    (-1, -1), // haut-gauche
];

fn check_word(grid: &[&[u8]], x: usize, y: usize, dx: isize, dy: isize, word: &[u8]) -> bool {
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);

    for (i, &c) in word.iter().enumerate() {
        let nx = x as isize + i as isize * dx;
        let ny = y as isize + i as isize * dy;

        if nx < 0 || ny < 0 || nx >= rows || ny >= cols {
            return false;
        }

        if grid[nx as usize][ny as usize] != c {
            return false;
        }
    }
    true
}

fn _p1(_input: &str) -> usize {
    let grid: Vec<&[u8]> = _input.lines()
        .map(str::as_bytes)
        .collect();

    let word = b"XMAS";
    let mut count = 0;

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            count += DIRECTIONS.iter()
                .filter(|&&(dx, dy)| check_word(&grid, x, y, dx, dy, word))
                .count();
        }
    }

    count
}

fn is_mas(s: &[u8]) -> bool {
    s == b"MAS" || s == b"SAM"
}

fn _p2(_input: &str) -> usize {
    let grid: Vec<&[u8]> = _input.lines()
        .map(str::as_bytes)
        .collect();

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut count = 0;

    for x in 0..rows.saturating_sub(2) {
        for y in 0..cols.saturating_sub(2) {
            let diag1 = [
                grid[x][y],
                grid[x + 1][y + 1],
                grid[x + 2][y + 2],
            ];

            let diag2 = [
                grid[x + 2][y],
                grid[x + 1][y + 1],
                grid[x][y + 2],
            ];

            if is_mas(&diag1) && is_mas(&diag2) {
                count += 1;
            }
        }
    }

    count
}


/*
fn _p1(_input: &str) -> usize {
    let sopa: Vec<&str> = _input.lines().collect();
    let filas = sopa.len();
    let columnas = sopa[0].len();
    let palabra = "XMAS";

    let mut total_apariciones = 0;

    let buscar_en_direccion = |x: usize, y: usize, dx: isize, dy: isize| -> bool {
        for (i, c) in palabra.chars().enumerate() {
            let nx = x as isize + i as isize * dx;
            let ny = y as isize + i as isize * dy;

            if nx < 0 || ny < 0 || nx >= filas as isize || ny >= columnas as isize {
                return false;
            }

            if sopa[nx as usize].chars().nth(ny as usize).unwrap() != c {
                return false;
            }
        }
        true
    };

    for x in 0..filas {
        for y in 0..columnas {

            let direcciones = [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ];

            for &(dx, dy) in &direcciones {
                if buscar_en_direccion(x, y, dx, dy) {
                    total_apariciones += 1;
                }
            }
        }
    }

    total_apariciones
}

fn _p2(_input: &str) -> usize {
    let sopa: Vec<&str> = _input.lines().collect();
    let filas = sopa.len();
    let columnas = sopa[0].len();
    let mut total_xmas = 0;

    let es_mas = |palabra: &str| -> bool {
        palabra == "MAS" || palabra == "SAM"
    };

    for x in 0..(filas - 2) {
        for y in 0..(columnas - 2) {
            let diagonal1: String = [
                sopa[x].chars().nth(y).unwrap(),
                sopa[x + 1].chars().nth(y + 1).unwrap(),
                sopa[x + 2].chars().nth(y + 2).unwrap(),
            ]
                .iter()
                .collect();

            let diagonal2: String = [
                sopa[x + 2].chars().nth(y).unwrap(),
                sopa[x + 1].chars().nth(y + 1).unwrap(),
                sopa[x].chars().nth(y + 2).unwrap(),
            ]
                .iter()
                .collect();

            if es_mas(&diagonal1) && es_mas(&diagonal2) {
                total_xmas += 1;
            }
        }
    }

    total_xmas
}*/

pub fn p1() -> usize {
    _p1(include_str!("../Inputs/d4.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("../Inputs/d4.txt"))
}

/*
#[cfg(test)]
mod tests {
    use crate::jour4::*;

    #[test]
    fn test_p1() {
        assert_eq!(18, _p1(include_str!("../Inputs/d4_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(9, _p2(include_str!("../Inputs/d4_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(2644, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(1952, p2());
    }
}*/