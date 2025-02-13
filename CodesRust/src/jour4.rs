fn _p1(_input: &str) -> usize {
    let sopa: Vec<&str> = _input.lines().collect();
    let filas = sopa.len();
    let columnas = sopa[0].len(); // Asumimos que todas las filas tienen el mismo número de columnas
    let palabra = "XMAS";

    let mut total_apariciones = 0;

    // Función para verificar si la palabra está en una dirección específica
    let buscar_en_direccion = |x: usize, y: usize, dx: isize, dy: isize| -> bool {
        for (i, c) in palabra.chars().enumerate() {
            let nx = x as isize + i as isize * dx;
            let ny = y as isize + i as isize * dy;

            // Verificar si las coordenadas están dentro de los límites de la matriz
            if nx < 0 || ny < 0 || nx >= filas as isize || ny >= columnas as isize {
                return false;
            }

            // Comparar el carácter actual
            if sopa[nx as usize].chars().nth(ny as usize).unwrap() != c {
                return false;
            }
        }
        true
    };

    // Buscar la palabra en todas las direcciones
    for x in 0..filas {
        for y in 0..columnas {
            // Direcciones posibles: (dx, dy)
            let direcciones = [
                (0, 1),   // Horizontal derecha
                (0, -1),  // Horizontal izquierda
                (1, 0),   // Vertical abajo
                (-1, 0),  // Vertical arriba
                (1, 1),   // Diagonal abajo-derecha
                (1, -1),  // Diagonal abajo-izquierda
                (-1, 1),  // Diagonal arriba-derecha
                (-1, -1), // Diagonal arriba-izquierda
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
    let columnas = sopa[0].len(); // Asumimos que todas las filas tienen el mismo número de columnas
    let mut total_xmas = 0;

    // Función para verificar si una palabra es "MAS" o "SAM"
    let es_mas = |palabra: &str| -> bool {
        palabra == "MAS" || palabra == "SAM"
    };

    // Buscar la forma de un X-MAS en todas las posiciones posibles
    for x in 0..(filas - 2) {
        for y in 0..(columnas - 2) {
            // Obtener las dos diagonales
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

            // Verificar si ambas diagonales forman un X-MAS
            if es_mas(&diagonal1) && es_mas(&diagonal2) {
                total_xmas += 1;
            }
        }
    }

    total_xmas
}

pub fn p1() -> usize {
    _p1(include_str!("../Inputs/d4.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("../Inputs/d4.txt"))
}

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
}