use std::collections::HashSet;
fn _p1(_input: &str) -> usize {
    // Parsear el mapa
    let mapa: Vec<Vec<char>> = _input.lines().map(|linea| linea.chars().collect()).collect();

    let filas = mapa.len();
    let columnas = mapa[0].len();

    // Direcciones y rotación: [arriba, derecha, abajo, izquierda]
    let direcciones = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Identificar la posición inicial y la dirección
    let mut pos_guardia = (0, 0);
    let mut direccion = 0; // 0 = arriba, 1 = derecha, 2 = abajo, 3 = izquierda

    for (fila, linea) in mapa.iter().enumerate() {
        for (col, &caracter) in linea.iter().enumerate() {
            match caracter {
                '^' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 0;
                }
                '>' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 1;
                }
                'v' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 2;
                }
                '<' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 3;
                }
                _ => {}
            }
        }
    }

    // Mantener un registro de las posiciones visitadas
    let mut posiciones_visitadas: HashSet<(isize, isize)> = HashSet::new();
    posiciones_visitadas.insert(pos_guardia);

    loop {
        let (dx, dy) = direcciones[direccion];
        let nueva_pos = (pos_guardia.0 + dx, pos_guardia.1 + dy);

        // Verificar si la nueva posición está fuera del mapa
        if nueva_pos.0 < 0
            || nueva_pos.1 < 0
            || nueva_pos.0 >= filas as isize
            || nueva_pos.1 >= columnas as isize
        {
            break;
        }

        // Verificar si hay un obstáculo en la nueva posición
        if mapa[nueva_pos.0 as usize][nueva_pos.1 as usize] == '#' {
            // Girar 90 grados a la derecha
            direccion = (direccion + 1) % 4;
        } else {
            // Avanzar y marcar la posición como visitada
            pos_guardia = nueva_pos;
            posiciones_visitadas.insert(pos_guardia);
        }
    }

    // Retornar el número de posiciones visitadas
    posiciones_visitadas.len()
}

fn _p2(_input: &str) -> usize {
    // Parsear el mapa
    let mut mapa: Vec<Vec<char>> = _input.lines().map(|linea| linea.chars().collect()).collect();

    let filas = mapa.len();
    let columnas = mapa[0].len();

    // Direcciones y rotación: [arriba, derecha, abajo, izquierda]
    let direcciones = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // Identificar la posición inicial y la dirección
    let mut pos_guardia = (0, 0);
    let mut direccion = 0; // 0 = arriba, 1 = derecha, 2 = abajo, 3 = izquierda

    for (fila, linea) in mapa.iter().enumerate() {
        for (col, &caracter) in linea.iter().enumerate() {
            match caracter {
                '^' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 0;
                }
                '>' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 1;
                }
                'v' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 2;
                }
                '<' => {
                    pos_guardia = (fila as isize, col as isize);
                    direccion = 3;
                }
                _ => {}
            }
        }
    }

    // Función para simular el movimiento del guardia y detectar bucles
    let es_bucle = |mapa: &Vec<Vec<char>>| -> bool {
        let mut posiciones_visitadas: HashSet<(isize, isize, usize)> = HashSet::new();
        let mut pos_actual = pos_guardia;
        let mut dir_actual = direccion;

        posiciones_visitadas.insert((pos_actual.0, pos_actual.1, dir_actual));

        loop {
            let (dx, dy) = direcciones[dir_actual];
            let nueva_pos = (pos_actual.0 + dx, pos_actual.1 + dy);

            // Verificar si la nueva posición está fuera del mapa
            if nueva_pos.0 < 0
                || nueva_pos.1 < 0
                || nueva_pos.0 >= filas as isize
                || nueva_pos.1 >= columnas as isize
            {
                return false; // El guardia salió del mapa
            }

            // Verificar si hay un obstáculo en la nueva posición
            if mapa[nueva_pos.0 as usize][nueva_pos.1 as usize] == '#' {
                // Girar 90 grados a la derecha
                dir_actual = (dir_actual + 1) % 4;
            } else {
                // Avanzar
                pos_actual = nueva_pos;
            }

            // Verificar si ya hemos visitado esta posición y dirección
            if !posiciones_visitadas.insert((pos_actual.0, pos_actual.1, dir_actual)) {
                return true; // Se detectó un bucle
            }
        }
    };

    // Probar todas las posiciones posibles para la nueva obstrucción
    let mut posiciones_validas = 0;

    for fila in 0..filas {
        for col in 0..columnas {
            if mapa[fila][col] == '.' && (fila as isize, col as isize) != pos_guardia {
                // Colocar una obstrucción temporal
                mapa[fila][col] = '#';

                // Verificar si causa un bucle
                if es_bucle(&mapa) {
                    posiciones_validas += 1;
                }

                // Remover la obstrucción temporal
                mapa[fila][col] = '.';
            }
        }
    }

    posiciones_validas
}

pub fn p1() -> usize {
    _p1(include_str!("d6.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("d6.txt"))
}

#[cfg(test)]
mod tests {
    use crate::d6::*;

    #[test]
    fn test_p1() {
        assert_eq!(41, _p1(include_str!("d6_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(6, _p2(include_str!("d6_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(5453, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(2188, p2());
    }
}