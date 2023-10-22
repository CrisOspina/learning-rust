use std::io;

fn main() {
    println!("- Data types");
    data_types();

    println!("- Scalar types");
    scalar_types();

    println!("- Compound types");
    compound_types();

    println!("Do you want to see an example? [y/n]");

    question_for_example()
}

fn question_for_example() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    match input.trim() {
        "y" => {
            println!("-- Example Data Types");
            example_data_types();
        },
        "n" => println!("Ok bye human, see you later!"),
        _ => println!("Invalid option")
    }
}

fn data_types() {
    let number: i32 = 3;
    let name: &str = "Cris";
    let is_valid: bool = true;
    
    println!("Number: {number}");
    println!("Name: {name}");
    println!("isValid: {is_valid}");
}

fn scalar_types() {
    println!("-- Integer Numbers");
    integer_number();

    println!("-- Floating Numbers");
    floating_numbers();

    println!("-- Numeric Operations");
    numeric_operations();

    println!("-- Boolean Type");
    boolean_type();

    println!("-- Character Type");
    charater_type();
}

fn integer_number() {
    // signed = i8, i16, i32, i64, i128, isize
    // unsigned = u8, u16, u32, u64, u128, usize

    let age_negative: i8 = -66;
    let age_positive: u8 = 66;

    println!("Age: {age_negative}");
    println!("Age: {age_positive}");
}

fn floating_numbers() {
    // f32, f64
    let pi: f32 = 3.141592;
    let e: f64 = 2.71828182845904523536028747135266249775724709369995;

    println!("Pi: {pi}");
    println!("E: {e}");
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");
    println!("Remainder: {remainder}");
}

fn boolean_type() {
    let is_valid: bool = true;
    let is_invalid: bool = false;

    println!("isValid: {is_valid}");
    println!("isInvalid: {is_invalid}");
}

fn charater_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {c}");
    println!("z: {z}");
    println!("heart_eyed_cat: {heart_eyed_cat}");
}

fn compound_types() {
    println!("-- Tuple Type");
    tuple_type();

    println!("-- Array Type");
    array_type();
}

fn tuple_type() {
    let tup: (i32, f64, i8) = (100, 6.5, 3);
    let (x, y, z) = tup;

    println!("x: {x}");
    println!("y: {y}");
    println!("z: {z}");

    let x = (400, 4.4, 1);
    let five_hundred = x.0;
    let four_point_four = x.1;
    let one = x.2;

    println!("five_hundred: {five_hundred}");
    println!("four_point_four: {four_point_four}");
    println!("one: {one}");
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("first: {first}");
    println!("second: {second}");
}

fn example_data_types() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index [1,2,3,4,5]");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}