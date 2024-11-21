use std::io;

fn main() {
    // Mensajes de prueba
    println!("");
    println!("------------------------  Calculadora con Chadust ------------------------ ");
    println!("------------------------      by Mz :P :P         ------------------------ ");
    println!("");

    loop {
        // Pedir dos números para hacer las operaciones
        println!("Por favor, ingresa el primer número:");

        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Error al leer la línea");
        let num1: f64 = num1.trim().parse().expect("Tenés que poner un número válido");

        println!("Ingresa el segundo número:");

        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Error al leer la línea");
        let num2: f64 = num2.trim().parse().expect("Ingresa un número válido pajin");

        // Operación para hacer
        println!("Elige una operación (+, -, *, /):");

        let mut operacion = String::new();
        io::stdin().read_line(&mut operacion).expect("Error al leer la línea");
        let operacion = operacion.trim();

        // Realiza la operación
        let resultado = match operacion {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Error: No se puede dividir entre cero.");
                    continue;  
                }
            }
            _ => {
                println!("Hiciste la operación como el orto pibe");
                continue; 
            }
        };

        println!("---------------------------------------------------------------------------");
        println!("El resultado de {} {} {} es: {}", num1, operacion, num2, resultado);
        println!("---------------------------------------------------------------------------");
        println!("");
        println!("¿Querés hacer otra opetación pancho? (s para sí, cualquier otra tecla para salir):");
        
        let mut respuesta = String::new();
        io::stdin().read_line(&mut respuesta).expect("Error al leer la línea");

        if respuesta.trim() != "s" {
            println!("Chau pete.");
            break;  
        }
    }
}
