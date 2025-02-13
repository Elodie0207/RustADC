//112.0ms
/*fn _p1(_input: &str) -> u64 {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in _input.lines() {
        let mut report = Vec::new();
        for value in line.split_whitespace() {
            let number: i32 = value.parse().unwrap();
            report.push(number);
        }
        reports.push(report);
    }

    let mut safe_count = 0;


    for report in &reports {
        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;

        for i in 0..report.len() - 1 {
            let diff = (report[i + 1] - report[i]).abs();


            if diff < 1 || diff > 3 {
                valid_differences = false;
                break;
            }


            if report[i + 1] > report[i] {
                is_decreasing = false;
            } else if report[i + 1] < report[i] {
                is_increasing = false;
            }
        }


        if (is_increasing || is_decreasing) && valid_differences {

            safe_count += 1;
        } else {

        }
    }


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
        reports.push(report);
    }

    let mut safe_count = 0;


    for report in &reports {
        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;

        for i in 0..report.len() - 1 {
            let diff = (report[i + 1] - report[i]).abs();

            if diff < 1 || diff > 3 {
                valid_differences = false;
                break;
            }

            if report[i + 1] > report[i] {
                is_decreasing = false;
            } else if report[i + 1] < report[i] {
                is_increasing = false;
            }
        }


        if (is_increasing || is_decreasing) && valid_differences {

            safe_count += 1;
        } else {

            let mut safe_after_removal = false;

            for i in 0..report.len() {

                let mut modified_report = report.clone();
                modified_report.remove(i);

                let mut is_increasing = true;
                let mut is_decreasing = true;
                let mut valid_differences = true;

                for j in 0..modified_report.len() - 1 {
                    let diff = (modified_report[j + 1] - modified_report[j]).abs();


                    if diff < 1 || diff > 3 {
                        valid_differences = false;
                        break;
                    }


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

                safe_count += 1;
            } else {

            }
        }
    }


    println!("Número total de reportes 'safe': {}", safe_count);
    safe_count
}

pub fn p1() -> u64 {
    _p1(include_str!("../Inputs/InputD2.txt"))
}

pub fn p2() -> u64 {
    _p2(include_str!("../Inputs/InputD2.txt"))
}
*/


//Environ 105ms
fn _p1(_input: &str) -> u64 {
    let mut safe_count = 0;

    for line in _input.lines() {
        let mut report = line
            .split_whitespace()
            .filter_map(|value| value.parse::<i32>().ok())
            .collect::<Vec<_>>();

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;

        for i in 0..report.len() - 1 {
            let diff = (report[i + 1] - report[i]).abs();

            // Si la différence est invalide, on arrête l'analyse de cette ligne.
            if diff < 1 || diff > 3 {
                valid_differences = false;
                break;
            }

            if report[i + 1] > report[i] {
                is_decreasing = false;
            } else if report[i + 1] < report[i] {
                is_increasing = false;
            }
        }

        // Si la ligne est valide, on l'ajoute au compteur
        if valid_differences && (is_increasing || is_decreasing) {
            safe_count += 1;
        }
    }

    safe_count
}

fn _p2(_input: &str) -> u64 {
    let mut safe_count = 0;

    for line in _input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|value| value.parse::<i32>().ok())
            .collect();

        let mut is_increasing = true;
        let mut is_decreasing = true;
        let mut valid_differences = true;

        // Première passe pour vérifier si le rapport est valide
        for i in 0..report.len() - 1 {
            let diff = (report[i + 1] - report[i]).abs();

            if diff < 1 || diff > 3 {
                valid_differences = false;
                break;
            }

            if report[i + 1] > report[i] {
                is_decreasing = false;
            } else if report[i + 1] < report[i] {
                is_increasing = false;
            }
        }

        if valid_differences && (is_increasing || is_decreasing) {
            safe_count += 1;
        } else {
            // Deuxième passe : on vérifie si un élément peut être retiré pour rendre la ligne valide
            let mut safe_after_removal = false;

            for i in 0..report.len() {
                let mut is_increasing = true;
                let mut is_decreasing = true;
                let mut valid_differences = true;

                // Vérification du rapport sans l'élément à l'index i
                for j in 0..report.len() - 1 {
                    if j == i {
                        continue; // On saute l'élément supprimé
                    }

                    let current = if j < i { report[j] } else { report[j + 1] };
                    let next = if j < i { report[j + 1] } else { report[j] };
                    let diff = (next - current).abs();

                    if diff < 1 || diff > 3 {
                        valid_differences = false;
                        break;
                    }

                    if next > current {
                        is_decreasing = false;
                    } else if next < current {
                        is_increasing = false;
                    }
                }

                if valid_differences && (is_increasing || is_decreasing) {
                    safe_after_removal = true;
                    break;
                }
            }

            if safe_after_removal {
                safe_count += 1;
            }
        }
    }

    println!("Número total de reportes 'safe': {}", safe_count);
    safe_count
}



pub fn p1() -> u64 {
    _p1(include_str!("../Inputs/InputD2.txt"))
}

pub fn p2() -> u64 {
    _p2(include_str!("../Inputs/InputD2.txt"))
}