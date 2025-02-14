//code opti


fn parse_mul_expression(input: &str) -> Option<(i32, i32)> {
    let end = input.find(')')?;
    let nums: Vec<&str> = input[..end].split(',').map(str::trim).collect();
    if nums.len() == 2 {
        Some((
            nums[0].parse().ok()?,
            nums[1].parse().ok()?
        ))
    } else {
        None
    }
}

fn _p1(_input: &str) -> i32 {
    _input
        .split("mul(")
        .skip(1)
        .filter_map(parse_mul_expression)
        .map(|(x, y)| x * y)
        .sum()
}

fn _p2(_input: &str) -> i32 {
    let mut sum = 0;
    let mut enabled = true;

    let mut chars = _input.as_bytes();
    let mut i = 0;

    while i < chars.len() {
        match &chars[i..] {
            [b'm', b'u', b'l', b'(', ..] if enabled => {
                if let Some((x, y)) = parse_mul_expression(
                    std::str::from_utf8(&chars[i+4..]).unwrap()
                ) {
                    sum += x * y;
                }
                i += 4;
            },
            [b'd', b'o', b'(', b')', ..] => {
                enabled = true;
                i += 4;
            },
            [b'd', b'o', b'n', b'\'', b't', b'(', b')', ..] => {
                enabled = false;
                i += 7;
            },
            _ => i += 1,
        }
    }

    sum
}



/*
fn _p1(_input: &str) -> i32 {
    let mut suma_total = 0;

    let partes: Vec<&str> = _input.split("mul(").collect();

    for parte in partes.iter().skip(1) {

        if let Some(cierre) = parte.find(')') {

            let seccion = &parte[..cierre];

            let numeros: Vec<&str> = seccion.split(',').map(|s| s.trim()).collect();
            if numeros.len() == 2 {
                if let (Ok(x), Ok(y)) = (numeros[0].parse::<i32>(), numeros[1].parse::<i32>()) {
                    suma_total += x * y;
                }
            }
        }
    }

    suma_total
}

fn _p2(_input: &str) -> i32 {
    let mut suma_total = 0;

    let mut mul_habilitado = true;

    let partes: Vec<&str> = _input.split("mul(").collect();

    for (i, parte) in partes.iter().enumerate() {

        if i > 0 {

            if let Some(cierre) = parte.find(')') {

                let seccion = &parte[..cierre];

                let numeros: Vec<&str> = seccion.split(',').map(|s| s.trim()).collect();
                if numeros.len() == 2 {
                    if mul_habilitado {
                        if let (Ok(x), Ok(y)) = (numeros[0].parse::<i32>(), numeros[1].parse::<i32>()) {
                            suma_total += x * y;
                        }
                    }
                }
            }
        }

        if parte.contains("do()") {
            mul_habilitado = true;
        } else if parte.contains("don't()") {
            mul_habilitado = false;
        }
    }

    suma_total
}
*/
pub fn p1() -> i32 {
    _p1(include_str!("../Inputs/d3.txt"))
}

pub fn p2() -> i32 {
    _p2(include_str!("../Inputs/d3.txt"))
}
/*
#[cfg(test)]
mod tests {
    use crate::jour3::*;

    #[test]
    fn test_p1() {
        assert_eq!(161, _p1(include_str!("../Inputs/d3_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(48, _p2(include_str!("../Inputs/d3_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(174561379, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(106921067, p2());
    }
}*/