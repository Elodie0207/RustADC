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

pub fn p1() -> i32 {
    _p1(include_str!("../Inputs/InputD3.txt"))
}

pub fn p2() -> i32 {
    _p2(include_str!("../Inputs/InputD3.txt"))
}
