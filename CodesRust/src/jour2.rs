fn _p1(_input: &str) -> u64 {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in _input.lines() {
        let mut report = Vec::new();
        for value in line.split_whitespace() {
            let number: i32 = value.parse().unwrap();
            report.push(number);
        }
        reports.push(report); // Añadir la línea completa al vector de vectores
    }

    let mut safe_count = 0;

    // Verificar cada línea si es creciente, decreciente y cumple con las diferencias
    for report in &reports {
        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;

        for i in 0..report.len() - 1 {
            let diff = (report[i + 1] - report[i]).abs(); // Diferencia absoluta entre números adyacentes

            // Verificar si las diferencias están entre 1 y 3
            if diff < 1 || diff > 3 {
                valid_differences = false;
                break;
            }

            // Verificar si la línea es creciente o decreciente
            if report[i + 1] > report[i] {
                is_decreasing = false;
            } else if report[i + 1] < report[i] {
                is_increasing = false;
            }
        }

        // Si cumple todas las condiciones, es "safe"
        if (is_increasing || is_decreasing) && valid_differences {
            //println!("Línea {:?} es 'safe'", report);
            safe_count += 1;
        } else {
            //println!("Línea {:?} no es 'safe'", report);
        }
    }

    // Imprimir el número total de reportes seguros
    //println!("Número total de reportes 'safe': {}", safe_count);
    safe_count
}

fn _p2(_input: &str) -> u64 {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in _input.lines() {
        let mut report = Vec::new();
        for value in line.split_whitespace() {
            let number: i32 = value.parse().unwrap();
            report.push(number);
        }
        reports.push(report); // Añadir la línea completa al vector de vectores
    }

    let mut safe_count = 0;

    // Verificar cada línea si es creciente, decreciente y cumple con las diferencias
    for report in &reports {
        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;

        for i in 0..report.len() - 1 {
            let diff = (report[i + 1] - report[i]).abs(); // Diferencia absoluta entre números adyacentes

            // Verificar si las diferencias están entre 1 y 3
            if diff < 1 || diff > 3 {
                valid_differences = false;
                break;
            }

            // Verificar si la línea es creciente o decreciente
            if report[i + 1] > report[i] {
                is_decreasing = false;
            } else if report[i + 1] < report[i] {
                is_increasing = false;
            }
        }

        // Si cumple todas las condiciones, es "safe"
        if (is_increasing || is_decreasing) && valid_differences {
            //println!("Línea {:?} es 'safe'", report);
            safe_count += 1;
        } else {
            // Intentar eliminar un solo nivel para ver si la secuencia se convierte en segura
            let mut safe_after_removal = false;

            for i in 0..report.len() {
                // Crear una nueva secuencia sin el elemento i
                let mut modified_report = report.clone();
                modified_report.remove(i);

                let mut is_increasing = true;
                let mut is_decreasing = true;
                let mut valid_differences = true;

                for j in 0..modified_report.len() - 1 {
                    let diff = (modified_report[j + 1] - modified_report[j]).abs();

                    // Verificar si las diferencias están entre 1 y 3
                    if diff < 1 || diff > 3 {
                        valid_differences = false;
                        break;
                    }

                    // Verificar si la línea es creciente o decreciente
                    if modified_report[j + 1] > modified_report[j] {
                        is_decreasing = false;
                    } else if modified_report[j + 1] < modified_report[j] {
                        is_increasing = false;
                    }
                }

                if (is_increasing || is_decreasing) && valid_differences {
                    safe_after_removal = true;
                    break;
                }
            }

            if safe_after_removal {
                //println!("Línea {:?} es 'safe' al eliminar un nivel", report);
                safe_count += 1;
            } else {
                // println!("Línea {:?} no es 'safe'", report);
            }
        }
    }

    // Imprimir el número total de reportes seguros
    println!("Número total de reportes 'safe': {}", safe_count);
    safe_count
}

pub fn p1() -> u64 {
    _p1(include_str!("d2.txt"))
}

pub fn p2() -> u64 {
    _p2(include_str!("d2.txt"))
}

#[cfg(test)]
mod tests {
    use crate::jour2::*;

    #[test]
    fn test_p1() {
        assert_eq!(2, _p1(include_str!("d2_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(31, _p2(include_str!("d2_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(1258579, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(717, p2());
    }
}