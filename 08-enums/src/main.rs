
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');

    let absent_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);
}
