fn main() {
    // Getting started with Rust
    // println!("Hello, world!");
    // primitive_data_types();
    // prueba_inmutabilidad();
    // compound_data_types();

    // ejercicio factorial
    let numero: u128 = 5;
    let resultado_factorial = get_factorial(numero);
    println!("el factorial de {} es {}", numero, resultado_factorial);

    // ejercicio numero primo
    let numero_2_primo: u128 = 29;
    let primo = get_if_primitive(numero_2_primo);
    if primo {
        println!("El numero {} es primo", numero_2_primo);
    } else {
        println!("El numero {} NO es primo", numero_2_primo);
    }
}

