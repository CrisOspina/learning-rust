fn main() {
    println!("Control flow");

    println!("if expressions");
    if_expressions();

    println!("loops");
    loops();
}

fn loops() {
    println!("for loops");
    loops_for();

    println!("while loops");
    loops_while();

    println!("loop loops");
    loops_loop();
}

fn loops_loop() {
    println!("loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loops_while() {
    println!("while");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loops_for() {
    println!("for");

    for number in (1..10).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn if_expressions() {
    if_expression_1();
    if_expression_2();
    if_expression_3();
}

fn if_expression_1() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_expression_2() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_expression_3() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}