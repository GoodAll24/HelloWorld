// obtener el factorial
fn get_factorial(num: u128) -> u128 {
    if num == 0 || num == 1 {
        1
    } else {
        let mut resultado = num;
        for i in (2..num).rev() {
            resultado = resultado * i;
        }
        resultado
    }
}

fn get_if_primitive(num: u128) -> bool {
    let mut primitivo = true;
    if num < 4 {
    } else {
        let num_2_sqr = num as f64;

        for i in (2..(num_2_sqr.sqrt() as u128) + 1) {
            if num % i == 0 {
                primitivo = false;
            }
        }
    }
    primitivo
}
