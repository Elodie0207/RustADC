use std::collections::HashSet;
// 69.4 s

/*fn _p1(_input: &str) -> usize {
    let mapa: Vec<Vec<char>> = _input.lines().map(|linea| linea.chars().collect()).collect();
    let filas = mapa.len();
    let columnas = mapa[0].len();

    // Almacenar las posiciones de las antenas
    let mut antenas = Vec::new();

    for (fila, linea) in mapa.iter().enumerate() {
        for (col, &caracter) in linea.iter().enumerate() {
            if caracter.is_alphanumeric() {
                antenas.push((caracter, fila as isize, col as isize));
            }
        }
    }

    // Calcular los antinodos
    let mut antinodos: HashSet<(isize, isize)> = HashSet::new();

    for i in 0..antenas.len() {
        let (freq1, x1, y1) = antenas[i];

        for j in 0..antenas.len() {
            if i != j {
                let (freq2, x2, y2) = antenas[j];

                // Las antenas deben tener la misma frecuencia
                if freq1 == freq2 {
                    let dx = x2 - x1;
                    let dy = y2 - y1;

                    // Verificar si una antena está exactamente al doble de distancia de la otra
                    let x3 = x1 - dx; // Antinodo en una dirección
                    let y3 = y1 - dy;
                    let x4 = x2 + dx; // Antinodo en la dirección opuesta
                    let y4 = y2 + dy;

                    // Verificar que las posiciones estén dentro del mapa
                    if x3 >= 0 && x3 < filas as isize && y3 >= 0 && y3 < columnas as isize {
                        antinodos.insert((x3, y3));
                    }
                    if x4 >= 0 && x4 < filas as isize && y4 >= 0 && y4 < columnas as isize {
                        antinodos.insert((x4, y4));
                    }
                }
            }
        }
    }

    // Devolver la cantidad de posiciones únicas con antinodos
    antinodos.len()
}

fn _p2(_input: &str) -> usize {
    let mapa: Vec<Vec<char>> = _input.lines().map(|linea| linea.chars().collect()).collect();
    let filas = mapa.len();
    let columnas = mapa[0].len();

    // Declaramos las antenas sin especificar el tipo explícitamente
    let mut antenas = Vec::new();

    // Almacenar las antenas con sus posiciones
    for (fila, linea) in mapa.iter().enumerate() {
        for (col, &caracter) in linea.iter().enumerate() {
            if caracter.is_alphanumeric() {
                // Rust infiere que el vector contiene tuplas (char, isize, isize)
                antenas.push((caracter, fila as isize, col as isize));
            }
        }
    }

    // Calcular antinodos
    let mut antinodos: HashSet<(isize, isize)> = HashSet::new();

    for i in 0..antenas.len() {
        let (freq1, x1, y1) = antenas[i];
        let mut tiene_pareja = false;

        for j in 0..antenas.len() {
            if i != j {
                let (freq2, x2, y2) = antenas[j];

                if freq1 == freq2 {
                    tiene_pareja = true;

                    // Calcular los incrementos para extender la línea recta
                    let dx = x2 - x1;
                    let dy = y2 - y1;

                    // Antinodo hacia un lado
                    let mut x = x1 - dx;
                    let mut y = y1 - dy;
                    while x >= 0 && x < filas as isize && y >= 0 && y < columnas as isize {
                        antinodos.insert((x, y));
                        x -= dx;
                        y -= dy;
                    }

                    // Antinodo hacia el otro lado
                    x = x2 + dx;
                    y = y2 + dy;
                    while x >= 0 && x < filas as isize && y >= 0 && y < columnas as isize {
                        antinodos.insert((x, y));
                        x += dx;
                        y += dy;
                    }
                }
            }
        }

        // Si tiene al menos otra antena con la misma frecuencia, se incluye la posición propia
        if tiene_pareja {
            antinodos.insert((x1, y1));
        }
    }

    antinodos.len()
}
*/

// 55.4 ms
fn _p1(_input: &str) -> usize {
    let mapa: Vec<Vec<char>> = _input.lines().map(|linea| linea.chars().collect()).collect();
    let filas = mapa.len();
    let columnas = mapa[0].len();

    // Almacenar las posiciones de las antenas
    let mut antenas = Vec::new();

    for (fila, linea) in mapa.iter().enumerate() {
        for (col, &caracter) in linea.iter().enumerate() {
            if caracter.is_alphanumeric() {
                antenas.push((caracter, fila as isize, col as isize));
            }
        }
    }

    // Calcular los antinodos
    let mut antinodos: HashSet<(isize, isize)> = HashSet::new();

    for i in 0..antenas.len() {
        let (freq1, x1, y1) = antenas[i];

        for j in (i + 1)..antenas.len() { // Evitar comparar el mismo par en ambas direcciones
            let (freq2, x2, y2) = antenas[j];

            // Las antenas deben tener la misma frecuencia
            if freq1 == freq2 {
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Verificar si una antena está exactamente al doble de distancia de la otra
                let x3 = x1 - dx; // Antinodo en una dirección
                let y3 = y1 - dy;
                let x4 = x2 + dx; // Antinodo en la dirección opuesta
                let y4 = y2 + dy;

                // Verificar que las posiciones estén dentro del mapa solo una vez
                if (x3 >= 0 && x3 < filas as isize && y3 >= 0 && y3 < columnas as isize)
                    || (x4 >= 0 && x4 < filas as isize && y4 >= 0 && y4 < columnas as isize)
                {
                    if x3 >= 0 && x3 < filas as isize && y3 >= 0 && y3 < columnas as isize {
                        antinodos.insert((x3, y3));
                    }
                    if x4 >= 0 && x4 < filas as isize && y4 >= 0 && y4 < columnas as isize {
                        antinodos.insert((x4, y4));
                    }
                }
            }
        }
    }

    // Devolver la cantidad de posiciones únicas con antinodos
    antinodos.len()
}


fn _p2(_input: &str) -> usize {
    let mapa: Vec<Vec<char>> = _input.lines().map(|linea| linea.chars().collect()).collect();
    let filas = mapa.len();
    let columnas = mapa[0].len();

    // Almacenar las antenas con sus posiciones
    let mut antenas = Vec::new();

    for (fila, linea) in mapa.iter().enumerate() {
        for (col, &caracter) in linea.iter().enumerate() {
            if caracter.is_alphanumeric() {
                antenas.push((caracter, fila as isize, col as isize));
            }
        }
    }

    // Calcular antinodos
    let mut antinodos: HashSet<(isize, isize)> = HashSet::new();

    for i in 0..antenas.len() {
        let (freq1, x1, y1) = antenas[i];
        let mut tiene_pareja = false;

        for j in (i + 1)..antenas.len() { // Evitar comparar el mismo par en ambas direcciones
            let (freq2, x2, y2) = antenas[j];

            if freq1 == freq2 {
                tiene_pareja = true;

                // Calcular los incrementos para extender la línea recta
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Antinodo hacia un lado
                let mut x = x1 - dx;
                let mut y = y1 - dy;
                while x >= 0 && x < filas as isize && y >= 0 && y < columnas as isize {
                    antinodos.insert((x, y));
                    x -= dx;
                    y -= dy;
                }

                // Antinodo hacia el otro lado
                x = x2 + dx;
                y = y2 + dy;
                while x >= 0 && x < filas as isize && y >= 0 && y < columnas as isize {
                    antinodos.insert((x, y));
                    x += dx;
                    y += dy;
                }
            }
        }

        // Si tiene al menos otra antena con la misma frecuencia, se incluye la posición propia
        if tiene_pareja {
            antinodos.insert((x1, y1));
        }
    }

    antinodos.len()
}


pub fn p1() -> usize {
    _p1(include_str!("../Inputs/InputD8.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("../Inputs/InputD8.txt"))
}



