mod modulo_integral;
use modulo_integral::tipos::{TabularIntegral};



fn main() {
    println!("--- Iniciando Generación de Tabla de Integración por Partes ---");

    // La expresión que contiene u y dv separados por un '*'
    let full_expression = "x^3*e^5x";

    // Intentamos separar la expresión en sus partes 'u' y 'dv'
    if let Some((u_initial_str, dv_initial_str)) = full_expression.split_once('*') {
        println!("Expresión inicial: '{}'", full_expression);
        println!("Factor u: '{}'", u_initial_str);
        println!("Factor dv: '{}'", dv_initial_str);

        // Creamos una instancia de TabularIntegral usando los factores separados.
        // Los vectores 'derivadas' e 'integrales' se inicializan vacíos y se llenarán en generar_tabla().
        let mut tabla = TabularIntegral {
            u: u_initial_str.to_string(),   // Asigna 'u' de la expresión
            dv: dv_initial_str.to_string(), // Asigna 'dv' de la expresión
            derivadas: Vec::new(),          // Vacío, se llenará
            integrales: Vec::new(),         // Vacío, se llenará
        };

        println!("\nGenerando tabla...");

        // Llamamos al método que genera las derivadas e integrales y las guarda en la tabla.
        // Este método usará internamente tus funciones `derivacion` e `integracion`
        // junto con `clasificar_expresion` y `clasificar_integral`.
        tabla.generar_tabla();

        // Finalmente, imprimimos la tabla generada.
        tabla.imprimir_tabla();

    } else {
        // Si la expresión no contiene un '*', no podemos separarla y mostramos un error.
        eprintln!("Error: La expresión '{}' no contiene un '*' para separar los factores u y dv. No se puede generar la tabla.", full_expression);
    }

    println!("\n--- Generación de Tabla Finalizada ---");
    
    


}
  

