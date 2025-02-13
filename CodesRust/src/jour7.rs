fn _p1(_input: &str) -> i64 {
    let mut total_resultado: i64 = 0;

    for linea in _input.lines() {
        let partes: Vec<&str> = linea.split(':').collect();
        if partes.len() != 2 {
            continue;
        }

        let valor_objetivo: i64 = partes[0].trim().parse().unwrap_or(0);
        let numeros: Vec<i64> = partes[1]
            .trim()
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let num_operadores = numeros.len() - 1;
        let mut operadores = vec!['+'; num_operadores];
        let mut encontrada = false;

        while !encontrada {
            let mut resultado = numeros[0];

            for i in 0..num_operadores {
                if operadores[i] == '+' {
                    resultado += numeros[i + 1];
                } else if operadores[i] == '*' {
                    resultado *= numeros[i + 1];
                }
            }

            if resultado == valor_objetivo {
                total_resultado += valor_objetivo;
                encontrada = true;
                break;
            }

            let mut indice = num_operadores - 1;
            while indice < num_operadores {
                if operadores[indice] == '+' {
                    operadores[indice] = '*';
                    break;
                } else {
                    operadores[indice] = '+';
                    if indice == 0 {
                        encontrada = true;
                    } else {
                        indice -= 1;
                    }
                }
            }
        }
    }

    total_resultado
}

fn _p2(_input: &str) -> i64 {
    let mut total_resultado: i64 = 0;

    for linea in _input.lines() {
        let partes: Vec<&str> = linea.split(':').collect();
        if partes.len() != 2 {
            continue;
        }

        let valor_objetivo: i64 = partes[0].trim().parse().unwrap_or(0);
        let numeros: Vec<i64> = partes[1]
            .trim()
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();

        let num_operadores = numeros.len() - 1;
        let mut operadores = vec!['+'; num_operadores];
        let mut encontrada = false;

        while !encontrada {
            let mut resultado = numeros[0];

            for i in 0..num_operadores {
                if operadores[i] == '+' {
                    resultado += numeros[i + 1];
                } else if operadores[i] == '*' {
                    resultado *= numeros[i + 1];
                } else if operadores[i] == '|' {
                    let concatenado = format!("{}{}", resultado, numeros[i + 1]);
                    resultado = concatenado.parse::<i64>().unwrap_or(0);
                }
            }

            if resultado == valor_objetivo {
                total_resultado += valor_objetivo;
                encontrada = true;
                break;
            }

            let mut indice = num_operadores - 1;
            while indice < num_operadores {
                if operadores[indice] == '+' {
                    operadores[indice] = '*';
                    break;
                } else if operadores[indice] == '*' {
                    operadores[indice] = '|';
                    break;
                } else {
                    operadores[indice] = '+';
                    if indice == 0 {
                        encontrada = true;
                    } else {
                        indice -= 1;
                    }
                }
            }
        }
    }

    total_resultado
}

pub fn p1() -> i64 {
    _p1(include_str!("../Inputs/d7.txt"))
}

pub fn p2() -> i64 {
    _p2(include_str!("../Inputs/d7.txt"))
}

#[cfg(test)]
mod tests {
    use crate::jour7::*;

    #[test]
    fn test_p1() {
        assert_eq!(3749, _p1(include_str!("../Inputs/d7_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(11387, _p2(include_str!("../Inputs/d7_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(1620690235709, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(145397611075341, p2());
    }
}