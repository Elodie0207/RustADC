use std::collections::{HashMap, HashSet, VecDeque};
//460ms
/*fn _p1(_input: &str) -> usize {

    let partes: Vec<&str> = _input.split("\r\n\r\n").collect();


    if partes.len() < 2 {
        eprintln!("Error: no se encontraron las dos secciones en la entrada.");
        return 0;
    }

    let reglas_str = partes[0].trim();
    let actualizaciones_str = partes[1].trim();






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
    _p1(include_str!("../Inputs/d5.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("../Inputs/d5.txt"))
}
*/






/*
- Réductions de la création de structures temporaires (on utilise split_once à la place de faire plusieurs manipulations)
- On initialise les vectures avec une capacité initiale estimée pour l'allocation de la mémoire
- On réduit le nombre de traverses de données en utilisant les HashMaps correctement.
- On réduit les étapes de validation
- Utilisations de références
 */
fn _p1(input: &str) -> usize {
    // Utiliser des références plutôt que de créer de nouveaux vecteurs
    let mut parts = input.split("\r\n\r\n");

    let rules_str = parts.next().unwrap_or("");
    let updates_str = parts.next().unwrap_or("");

    // Pré-allouer les vecteurs avec une capacité estimée
    let mut rules = Vec::with_capacity(rules_str.lines().count());
    for line in rules_str.lines() {
        if let Some((x, y)) = line.split_once('|') {
            if let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) {
                rules.push((x, y));
            }
        }
    }

    let mut central_pages_sum = 0;

    // Traiter les mises à jour ligne par ligne sans stocker le vecteur complet
    for line in updates_str.lines() {
        let mut positions = HashMap::with_capacity(16); // Pré-allouer avec une taille raisonnable
        let mut pages: Vec<u32> = line
            .split(',')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

        if pages.is_empty() {
            continue;
        }

        // Remplir la HashMap en une seule passe
        for (i, &page) in pages.iter().enumerate() {
            positions.insert(page, i);
        }

        let mut is_valid = true;
        for &(x, y) in &rules {
            if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
                if pos_x >= pos_y {
                    is_valid = false;
                    break;
                }
            }
        }

        if is_valid {
            let central_page = pages[pages.len() / 2];
            central_pages_sum += central_page;
        }
    }

    central_pages_sum as usize
}

fn _p2(input: &str) -> usize {
    let mut parts = input.split("\r\n\r\n");

    let rules_str = parts.next().unwrap_or("");
    let updates_str = parts.next().unwrap_or("");

    let mut rules = Vec::with_capacity(rules_str.lines().count());
    for line in rules_str.lines() {
        if let Some((x, y)) = line.split_once('|') {
            if let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) {
                rules.push((x, y));
            }
        }
    }

    let mut central_pages_sum = 0;

    for line in updates_str.lines() {
        let pages: Vec<u32> = line
            .split(',')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

        if pages.is_empty() {
            continue;
        }

        let mut positions = HashMap::with_capacity(pages.len());
        for (i, &page) in pages.iter().enumerate() {
            positions.insert(page, i);
        }

        let mut is_valid = true;
        for &(x, y) in &rules {
            if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
                if pos_x >= pos_y {
                    is_valid = false;
                    break;
                }
            }
        }

        if !is_valid {
            let mut graph = HashMap::with_capacity(pages.len());
            let mut in_degree = HashMap::with_capacity(pages.len());

            for &(x, y) in &rules {
                if pages.contains(&x) && pages.contains(&y) {
                    graph.entry(x).or_insert_with(HashSet::new).insert(y);
                    *in_degree.entry(y).or_insert(0) += 1;
                    in_degree.entry(x).or_insert(0);
                }
            }

            let mut ordered = Vec::with_capacity(pages.len());
            let mut queue = VecDeque::with_capacity(pages.len());

            for &page in &pages {
                if in_degree.get(&page).copied().unwrap_or(0) == 0 {
                    queue.push_back(page);
                }
            }

            while let Some(current) = queue.pop_front() {
                ordered.push(current);
                if let Some(neighbors) = graph.get(&current) {
                    for &neighbor in neighbors {
                        if pages.contains(&neighbor) {
                            let count = in_degree.get_mut(&neighbor).unwrap();
                            *count -= 1;
                            if *count == 0 {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }

            // Ajouter les pages manquantes
            for &page in &pages {
                if !ordered.contains(&page) {
                    ordered.push(page);
                }
            }

            if !ordered.is_empty() {
                central_pages_sum += ordered[ordered.len() / 2];
            }
        }
    }

    central_pages_sum as usize
}

pub fn p1() -> usize {
    _p1(include_str!("../Inputs/d5.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("../Inputs/d5.txt"))
}