fn main() {
    let input: &str = "hello";
    let output: String = reverse_string(input);

    assert_eq!(output, "olleh");
    println!("{} -> {}", input, output);
}

pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}