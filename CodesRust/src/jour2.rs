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

        let is_valid = |report: &[i32]| -> bool {
            let mut is_increasing = true;
            let mut is_decreasing = true;

            for i in 0..report.len() - 1 {
                let diff = (report[i + 1] - report[i]).abs();

                if diff < 1 || diff > 3 {
                    return false;
                }

                if report[i + 1] > report[i] {
                    is_decreasing = false;
                } else if report[i + 1] < report[i] {
                    is_increasing = false;
                }
            }
            is_increasing || is_decreasing
        };

        if is_valid(&report) {
            safe_count += 1;
        } else {
            let mut safe_after_removal = false;

            for i in 0..report.len() {
                let mut modified_report = report.clone();
                modified_report.remove(i);

                if is_valid(&modified_report) {
                    safe_after_removal = true;
                    break;
                }
            }

            if safe_after_removal {
                safe_count += 1;
            }
        }
    }

    println!("Numero total: {}", safe_count);
    safe_count
}

pub fn p1() -> u64 {
    _p1(include_str!("../Inputs/d2.txt"))
}

pub fn p2() -> u64 {
    _p2(include_str!("../Inputs/d2.txt"))
}