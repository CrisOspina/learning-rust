pub fn unrecoverable() {
    panic!("crash and burn")
}

pub fn backtrace() {
    let v = vec![1, 2, 3];
    v[99];
}

pub fn recoverable() -> std::fs::File {
    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    return greeting_file;
}

pub fn recoverable_clear() -> std::fs::File {
    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    return greeting_file;
}

pub fn recover_with_expect() -> std::fs::File {
    use std::fs::File;
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");
    return greeting_file;
}
