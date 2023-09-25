fn main() {
    println!("convert fahrenheit to celsius");
    let fahrenheit: f64 = 100.0;

    let celsius = convert_fahrenheit_to_celsius(fahrenheit);
    println!("{} fahrenheit is {} celsius", fahrenheit, celsius);

    assert_eq!(convert_fahrenheit_to_celsius(fahrenheit), 37.77777777777778);
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}