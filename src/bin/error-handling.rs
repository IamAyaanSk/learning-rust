use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // ---------- Panic Macro ---------- //
    // We can panic in case of an unrecoverable error using panic macro
    fn _panic_at_12(val: i32) {
        if val == 12 {
            panic!("Don't call 12");
        };
    }
    // panic_at_12(12); This call will result in exiting the process with provided error message

    // ----------- Handling recoverable errors ----------- //
    // We have Result<T,E> enum

    let _file = File::open("generated/hello.txt");
    let _file = match _file {
        Ok(file) => file,

        // We can pani/handle error here
        // Create a new file if file was not found
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("generated/hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file {:?}", error),
            },
            other_error => panic!("Error opening a file {:?}", other_error),
        },
    };

    // We can use unwrap_or_else for better readability
    let _f = File::open("generated/hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("generated/hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Error opening a file {:?}", error);
        }
    });

    // unwrap() --> Returns the value in Ok but panics if error
    // expect() --> Same as unwrap but enables us to provide custom error message

    // Error Propogation
    fn _read_file(path: &str) -> Result<String, io::Error> {
        let mut file_content = String::new();
        // The ? operator says return the value to the variable if success or return the error from function for Result/Option
        File::open(path)?.read_to_string(&mut file_content)?;
        Ok(file_content)
    }

    // Now we can handle errors as usual
}
