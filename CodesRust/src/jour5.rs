use std::collections::{HashMap, HashSet, VecDeque};
fn _p1(_input: &str) -> usize {
    // Separar las dos partes (reglas y actualizaciones)
    let partes: Vec<&str> = _input.split("\r\n\r\n").collect();

    // Validar que haya al menos dos partes
    if partes.len() < 2 {
        eprintln!("Error: no se encontraron las dos secciones en la entrada.");
        return 0;
    }

    let reglas_str = partes[0].trim();
    let actualizaciones_str = partes[1].trim();

    // Imprimir las partes para depuración
    //println!("Reglas encontradas:\n{}", reglas_str);
    //println!("Actualizaciones encontradas:\n{}", actualizaciones_str);

    // Procesar las reglas
    let mut reglas = Vec::new();
    for linea in reglas_str.lines() {
        let partes: Vec<&str> = linea.split('|').collect();
        if partes.len() == 2 {
            if let (Ok(x), Ok(y)) = (partes[0].parse::<u32>(), partes[1].parse::<u32>()) {
                reglas.push((x, y));
            } else {
                eprintln!("Regla inválida ignorada: {}", linea);
            }
        } else {
            eprintln!("Formato de regla incorrecto: {}", linea);
        }
    }

    // Procesar las actualizaciones
    let mut actualizaciones = Vec::new();
    for linea in actualizaciones_str.lines() {
        let mut paginas = Vec::new();
        for num in linea.split(',') {
            if let Ok(pagina) = num.parse::<u32>() {
                paginas.push(pagina);
            } else {
                eprintln!("Número inválido en actualización: {}", num);
            }
        }
        if !paginas.is_empty() {
            actualizaciones.push(paginas);
        }
    }

    // Imprimir datos procesados para depuración
    //println!("Reglas procesadas: {:?}", reglas);
    //println!("Actualizaciones procesadas: {:?}", actualizaciones);

    // Verificar las actualizaciones y calcular la suma de las páginas centrales
    let mut suma_paginas_centrales = 0;

    for actualizacion in actualizaciones {
        // Crear un mapa para almacenar las posiciones de cada página en la actualización
        let mut posiciones = HashMap::new();
        for (i, &pagina) in actualizacion.iter().enumerate() {
            posiciones.insert(pagina, i);
        }

        // Verificar si la actualización cumple con las reglas
        let mut es_valida = true;
        for (x, y) in &reglas {
            if let (Some(&pos_x), Some(&pos_y)) = (posiciones.get(x), posiciones.get(y)) {
                if pos_x >= pos_y {
                    es_valida = false;
                    break;
                }
            }
        }

        // Sumar la página central si la actualización es válida
        if es_valida {
            let len = actualizacion.len();
            let pagina_central = actualizacion[len / 2];
            suma_paginas_centrales += pagina_central;
        }
    }

    suma_paginas_centrales as usize
}

fn _p2(_input: &str) -> usize {
    let partes: Vec<&str> = _input.split("\r\n\r\n").collect();

    if partes.len() < 2 {
        eprintln!("Error: no se encontraron las dos secciones en la entrada.");
        return 0;
    }

    let reglas_str = partes[0];
    let actualizaciones_str = partes[1];

    let mut reglas = Vec::new();
    for linea in reglas_str.lines() {
        let partes: Vec<&str> = linea.split('|').collect();
        if partes.len() == 2 {
            if let (Ok(x), Ok(y)) = (partes[0].parse::<u32>(), partes[1].parse::<u32>()) {
                reglas.push((x, y));
            }
        }
    }

    let mut actualizaciones = Vec::new();
    for linea in actualizaciones_str.lines() {
        let mut paginas = Vec::new();
        for num in linea.split(',') {
            if let Ok(pagina) = num.parse::<u32>() {
                paginas.push(pagina);
            }
        }
        if !paginas.is_empty() {
            actualizaciones.push(paginas);
        }
    }

    let mut suma_paginas_centrales = 0;

    for actualizacion in actualizaciones {
        let mut posiciones = HashMap::new();
        for (i, &pagina) in actualizacion.iter().enumerate() {
            posiciones.insert(pagina, i);
        }

        let mut es_valida = true;
        for (x, y) in &reglas {
            if let (Some(&pos_x), Some(&pos_y)) = (posiciones.get(x), posiciones.get(y)) {
                if pos_x >= pos_y {
                    es_valida = false;
                    break;
                }
            }
        }

        if !es_valida {
            println!("Actualización incorrecta encontrada: {:?}", actualizacion);

            // Reordenar la actualización respetando las reglas
            let mut grafo: HashMap<u32, HashSet<u32>> = HashMap::new();
            let mut in_degree: HashMap<u32, usize> = HashMap::new();

            for &(x, y) in &reglas {
                if actualizacion.contains(&x) && actualizacion.contains(&y) {
                    grafo.entry(x).or_insert_with(HashSet::new).insert(y);
                    *in_degree.entry(y).or_insert(0) += 1;
                    in_degree.entry(x).or_insert(0);
                }
            }

            let mut ordenado = Vec::new();
            let mut cola = VecDeque::new();

            for &pagina in &actualizacion {
                if in_degree.get(&pagina).copied().unwrap_or(0) == 0 {
                    cola.push_back(pagina);
                }
            }

            while let Some(actual) = cola.pop_front() {
                ordenado.push(actual);
                if let Some(vecinos) = grafo.get(&actual) {
                    for &vecino in vecinos {
                        if actualizacion.contains(&vecino) {
                            *in_degree.get_mut(&vecino).unwrap() -= 1;
                            if in_degree[&vecino] == 0 {
                                cola.push_back(vecino);
                            }
                        }
                    }
                }
            }

            // Agregar cualquier página que falte
            for &pagina in &actualizacion {
                if !ordenado.contains(&pagina) {
                    ordenado.push(pagina);
                }
            }

            println!("Actualización reordenada: {:?}", ordenado);

            if !ordenado.is_empty() {
                let len = ordenado.len();
                let pagina_central = ordenado[len / 2];
                println!("Página central: {}", pagina_central);
                suma_paginas_centrales += pagina_central;
            } else {
                eprintln!("Advertencia: No se pudo calcular la página central, la lista ordenada está vacía.");
            }
        }
    }

    suma_paginas_centrales as usize
}

pub fn p1() -> usize {
    _p1(include_str!("d5.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("d5.txt"))
}

#[cfg(test)]
mod tests {
    use crate::jour5::*;

    #[test]
    fn test_p1() {
        assert_eq!(143, _p1(include_str!("d5_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(123, _p2(include_str!("d5_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(5509, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(4407, p2());
    }
}