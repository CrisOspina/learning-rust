fn main() {
    println!("Functions!!");
    another_function();

    println!("Functions with parameters!!");
    function_with_parameter(5);

    println!("Statements and expressions!!");
    statements_and_expressions();

    println!("Functions with return!!");
    let x = five_with_return();
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn statements_and_expressions() {
    let y = {
        // Magic
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five_with_return() -> i32 {
    5
}