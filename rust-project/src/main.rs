// Importación de la biblioteca estándar para usar HashMap
use std::collections::HashMap;

// Definición de una estructura Car (coche) con campos color, motor, techo y age
#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

// Definición de una enumeración Transmission (transmisión) con diferentes modos de transmisión
#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

// Definición de una enumeración Age (edad) para representar si un coche es nuevo o usado
#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Función car_quality para determinar la calidad de un coche en función de las millas recorridas
fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        // Si el coche tiene millas, se considera "Used" (usado)
        return (Age::Used, miles);
    }
    // Si el coche no tiene millas, se considera "New" (nuevo)
    (Age::New, miles)
}

// Función car_factory para construir un objeto Car a partir de un pedido y millas
fn car_factory(order: i32, miles: u32) -> Car {
    // Definición de colores disponibles
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Lógica para determinar el color del coche en función del pedido
    let mut color = order as usize;
    while color > 4 {
        color = color - 4;
    }

    // Lógica para determinar el tipo de motor y el techo en función del pedido
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }

    // Construcción del objeto Car
    Car {
        color: String::from(colors[(color - 1) as usize]),
        motor,
        roof,
        age: car_quality(miles),
    }
}

fn main() {
    // Creación de un HashMap llamado "orders" para almacenar pedidos de coches
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Declaración de una variable mutable "car" de tipo Car
    let mut car: Car;

    // Inicialización de millas en 0
    let mut miles = 0;

    // Bucle for para crear 11 pedidos de coches
    for order in 1..12 {
        // Llamada a la función car_factory para crear un coche y agregarlo al HashMap
        car = car_factory(order, miles);
        orders.insert(order, car);

        // Impresión de detalles del pedido desde el HashMap
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Actualización de las millas para variedad en los pedidos
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }

    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];
}

