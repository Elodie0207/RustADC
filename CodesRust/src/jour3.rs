fn _p1(_input: &str) -> i32 {
    let mut suma_total = 0;

    // Dividir el contenido en posibles instrucciones
    let partes: Vec<&str> = _input.split("mul(").collect();

    // Analizar cada parte después de un "mul("
    for parte in partes.iter().skip(1) {
        // Buscar el cierre del paréntesis
        if let Some(cierre) = parte.find(')') {
            // Extraer el contenido dentro de los paréntesis
            let seccion = &parte[..cierre];

            // Verificar si la sección tiene exactamente un número, una coma, y otro número
            let numeros: Vec<&str> = seccion.split(',').map(|s| s.trim()).collect();
            if numeros.len() == 2 {
                // Intentar convertir ambos valores a enteros
                if let (Ok(x), Ok(y)) = (numeros[0].parse::<i32>(), numeros[1].parse::<i32>()) {
                    suma_total += x * y;
                }
            }
        }
    }

    suma_total
}

fn _p2(_input: &str) -> i32 {
    // Variable para acumular la suma de los productos
    let mut suma_total = 0;

    // Estado actual de las instrucciones mul (inicialmente habilitadas)
    let mut mul_habilitado = true;

    // Dividir el contenido en posibles instrucciones
    let partes: Vec<&str> = _input.split("mul(").collect();

    for (i, parte) in partes.iter().enumerate() {
        // Si no es la primera parte, procesamos como una posible instrucción mul
        if i > 0 {
            // Buscar el cierre del paréntesis
            if let Some(cierre) = parte.find(')') {
                // Extraer el contenido dentro de los paréntesis
                let seccion = &parte[..cierre];

                // Verificar si la sección tiene exactamente un número, una coma, y otro número
                let numeros: Vec<&str> = seccion.split(',').map(|s| s.trim()).collect();
                if numeros.len() == 2 {
                    // Procesar si las instrucciones mul están habilitadas
                    if mul_habilitado {
                        if let (Ok(x), Ok(y)) = (numeros[0].parse::<i32>(), numeros[1].parse::<i32>()) {
                            suma_total += x * y;
                        }
                    }
                }
            }
        }

        // Buscar las instrucciones "do()" o "don't()" en esta parte
        if parte.contains("do()") {
            mul_habilitado = true;
        } else if parte.contains("don't()") {
            mul_habilitado = false;
        }
    }

    suma_total
}

pub fn p1() -> i32 {
    _p1(include_str!("../Inputs/d3.txt"))
}

pub fn p2() -> i32 {
    _p2(include_str!("../Inputs/d3.txt"))
}

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
}