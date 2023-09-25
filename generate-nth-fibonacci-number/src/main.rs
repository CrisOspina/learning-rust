fn main() {
    let n = 10;
    let fib_n = generate_nth_fibonacci_number_recursive(n);
    println!("The {}th Fibonacci number is {}", n, fib_n);
    
    assert_eq!(generate_nth_fibonacci_number_recursive(10), 55);
}

fn generate_nth_fibonacci_number_recursive(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let previous = generate_nth_fibonacci_number_recursive(n - 1);
    let previous_previous = generate_nth_fibonacci_number_recursive(n - 2);

    println!("previous: {}", previous);
    println!("previous_previous: {}", previous_previous);

    return previous + previous_previous;
}