struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user = build_user(
        String::from("hela@gmail.com"), 
        String::from("hela")
    );

    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("active: {}", user.active);
    println!("sign_in_count: {}", user.sign_in_count);

    let black = Color(6, 6, 6);
    let origin = Point(2, 3, 2);

    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}