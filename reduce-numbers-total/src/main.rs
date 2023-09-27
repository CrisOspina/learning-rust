mod user;

use std::io;

/**
 * Calcule la suma de todos los números en un vector de enteros. Permitir que el usuario ingrese los números. Al final, muestra la suma total.
*/

fn main() {
    let mut user: user::User = user::User {
        numbers_added: Vec::new(),
    };
    
    while user.numbers_added.len() < user::MAX_NUMBERS {
        println!("Numbers: {:?}", user.numbers_added);
        println!("Add a number: ");

        let number = input_user();
        user.numbers_added.push(number);

        println!("{} added!", number);
    }

    println!("Numbers: {:?}", user.numbers_added);
    println!("Total_numbers: {}", user.total_numbers());
}

fn input_user() -> i8 {
    let mut input: String = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let number: Result<i8, std::num::ParseIntError> = input.trim().parse();

            match number {
                Ok(_) => return number.unwrap(),
                Err(_) => {
                    println!("Nahh, try again, must be a number: {}", input);
                    return input_user();
                }
            }             
        }
        Err(error) => println!("error: {error}"),
    }

    return 0;
}
