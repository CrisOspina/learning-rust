fn main() {
    println!("The Slice Type");
    
    let s = String::from("hello world");
    let word = first_word(&s);

    println!("The first word is: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    println!("The slice is: {:?}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }    

    &s[..]
}
