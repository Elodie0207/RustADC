
fn _p1(_input: &str) -> i64 {
    // Parsear el mapa en archivos y espacios libres
    let mut layout = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    for ch in _input.chars() {
        let size = ch.to_digit(10).unwrap() as usize;
        if is_file {
            layout.extend(vec![file_id as i64; size]); // Bloques del archivo
            file_id += 1;
        } else if size > 0 {
            layout.extend(vec![-1; size]); // Espacios libres representados como -1
        }
        is_file = !is_file;
    }

    // Imprimir el layout inicial para depuración
    //println!("Layout inicial: {:?}", layout);

    // Compactar el disco con `for` e `if`
    for i in 0..layout.len() {
        if layout[i] == -1 {
            // Buscar el último número válido desde la derecha
            for j in (i + 1..layout.len()).rev() {
                if layout[j] != -1 {
                    // Mover el número al espacio vacío
                    layout[i] = layout[j];
                    layout[j] = -1; // Marcar la posición original como vacía
                    break;
                }
            }
        }
    }

    // Imprimir el layout después de la compactación para depuración
    //println!("Layout después de la compactación: {:?}", layout);

    // Calcular el checksum
    let mut checksum: i64 = 0;
    for (i, &block) in layout.iter().enumerate() {
        if block != -1 { // Ignorar bloques vacíos
            let contribucion = i as i64 * block;
            //println!("Posición: {}, Bloque: {}, Contribución al checksum: {}", i, block, contribucion);
            checksum += contribucion;
        }
    }

    // Imprimir el checksum final para depuración
    //println!("Checksum final: {}", checksum);

    checksum
}

fn _p2(_input: &str) -> i64 {
    // Parsear el mapa en archivos y espacios libres
    let mut layout = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;

    for ch in _input.chars() {
        let size = ch.to_digit(10).unwrap() as usize;
        if is_file {
            layout.extend(vec![file_id as i64; size]); // Bloques del archivo
            file_id += 1;
        } else if size > 0 {
            layout.extend(vec![-1; size]); // Espacios libres representados como -1
        }
        is_file = !is_file;
    }

    // Imprimir el layout inicial para depuración
    //println!("Layout inicial: {:?}", layout);

    // Compactar el disco moviendo archivos completos
    for current_file in (0..file_id as i64).rev() {
        // Obtener las posiciones actuales del archivo
        let mut file_positions: Vec<usize> = layout
            .iter()
            .enumerate()
            .filter_map(|(i, &block)| if block == current_file { Some(i) } else { None })
            .collect();

        // Si no hay bloques del archivo, continuar con el siguiente
        if file_positions.is_empty() {
            continue;
        }

        let file_size = file_positions.len();
        let file_start = file_positions[0]; // Primera posición del archivo

        // Buscar el primer espacio libre suficientemente grande a la izquierda del archivo
        let mut found_space = None;
        let mut current_space_start = None;
        let mut current_space_size = 0;

        for i in 0..file_start {
            if layout[i] == -1 {
                // Espacio libre
                if current_space_start.is_none() {
                    current_space_start = Some(i);
                }
                current_space_size += 1;

                if current_space_size == file_size {
                    found_space = current_space_start;
                    break;
                }
            } else {
                // Bloque ocupado, reiniciar la búsqueda de espacio
                current_space_start = None;
                current_space_size = 0;
            }
        }

        if let Some(start) = found_space {
            // Colocar el archivo en el nuevo espacio
            for i in 0..file_size {
                layout[start + i] = current_file;
            }
            // Marcar los bloques originales como libres
            for &pos in &file_positions {
                layout[pos] = -1;
            }
        }
    }

    // Imprimir el layout después de la compactación para depuración
    //println!("Layout después de la compactación: {:?}", layout);

    // Calcular el checksum
    let mut checksum: i64 = 0;
    for (i, &block) in layout.iter().enumerate() {
        if block != -1 { // Ignorar bloques vacíos
            let contribucion = i as i64 * block;
            println!(
                "Posición: {}, Bloque: {}, Contribución al checksum: {}",
                i, block, contribucion
            );
            checksum += contribucion;
        }
    }

    // Imprimir el checksum final para depuración
    //println!("Checksum final: {}", checksum);

    checksum
}



pub fn p1() -> i64 {
    _p1(include_str!("d9.txt"))
}

pub fn p2() -> i64 {
    _p2(include_str!("d9.txt"))
}

#[cfg(test)]
mod tests {
    use crate::d9::*;

    #[test]
    fn test_p1() {
        assert_eq!(1928, _p1(include_str!("d9_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(2858, _p2(include_str!("d9_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(6415184586041, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(6436819084274, p2());
    }
}