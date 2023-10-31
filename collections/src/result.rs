use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

/**
 * https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html
 */

pub fn go() {
    // panic!("crash and burn")

    // let v = vec![1, 2, 3];

    // v[99];
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    match read_username_from_file() {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Read username failed: {err}"),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    //
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
