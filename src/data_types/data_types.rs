fn primitive_data_types() {
    // numeros enteros
    let numero1: i8 = 1;
    let numero2: i16 = 12;
    let numero3: i32 = 13;
    let numero4: i64 = 111;
    let numero5: i128 = 143124;
    println!("tipos de datos numericos enteros...");
    println!(
        "{}  {} {} {} {}",
        numero1, numero2, numero3, numero4, numero5
    );

    // numeros flotantes (decimales)
    // let numero1: f8 = 1.0;  // no existe
    // let numero2: f16 = 12.0;  // no existe
    let numerof1: f32 = 13.0;
    let numerof2: f64 = 111.0;
    let numerof3: f64 = numerof2 / 442.0;
    let numerof4: f32 = numerof1 + 23.5432;
    // let numero5: f128 = 143124.0;  // no existe
    println!("tipos de datos numericos flotantes...");
    println!("{}  {} {} {}", numerof1, numerof2, numerof3, numerof4);

    //booleans
    let expresion: bool = true;
    println!("{}", expresion);

    // chars
    let character: char = 't';
    println!("{}", character);
    // texto
    let texto: &str = "texto";
    println!("{}", texto);
}

fn compound_data_types() {
    // tuple
    let tupla: (&str, i8, f32) = ("Alden", 23, 1.85);

    println!(
        "Nombre: {}\n Edad: {}\n Estatura: {}",
        tupla.0, tupla.1, tupla.2
    );

    // Array
    let arr: [i16; 5];
}

fn prueba_inmutabilidad() {
    let var1: i8 = 1; // 'variable' inmutable
    let mut var2: i8 = 3; // variable mutable

    // var1 = 2; // Error de mutabilidad
    var2 = 5;

    println!("Variable inmutable --> {}", var1);
    println!("Variable mutable --> {}", var2);
}
