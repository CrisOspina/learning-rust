fn main() {
    println!("mutate variables");
    mutate_variables();
    
    println!("constants");
    constants();

    println!("shadowing");
    shadowing();
}

fn mutate_variables() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");
}

fn constants() {
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {MAX_POINTS}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}