use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // first example
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // second example
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file {:?}", other_error)
    //         }
    //     },
    // };

    // third example
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt")
    //             .unwrap_or_else(|error| panic!("Problem creating the file: {:?}", error))
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // Fourth example
    // Unwarp wil call the panic! macro for us. example
    // let greeting_file = File::open("hello.txt").unwrap();

    // Fifth example
    // Similarly, the expect method lets us also chooses the panic! error message.
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
}
