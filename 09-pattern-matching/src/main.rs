#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coins: Vec<Coin> = vec![Coin::Penny, Coin::Nickel, Coin::Dime];

    for coin in coins {
        println!("Value {}", value_in_cents(coin));
    }

    assert_eq!(value_in_cents(Coin::Penny), 1);
    assert_eq!(value_in_cents(Coin::Nickel), 5);
    assert_eq!(value_in_cents(Coin::Dime), 10);

    let with_state = Coin::Quarter(UsState::Alaska);
    println!("Value {}", value_in_cents_2(with_state));
    assert_eq!(value_in_cents_2(Coin::Quarter(UsState::Alaska)), 25);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        _ => 0,
    }
}

fn value_in_cents_2(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
