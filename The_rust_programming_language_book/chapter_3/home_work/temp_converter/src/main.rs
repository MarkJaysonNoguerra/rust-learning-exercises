use std::io;

fn main() {
    'end_program: loop {
        println!("\n\nWhat to convert?\n\n");
        println!("Type 1 for fahreheit to celsius");
        println!("Type 2 for celsius to fahrenheit");

        let mut selection: String = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read selection");

        let selected: i32 = match selection.trim().parse() {
            Ok(num) => {
                if num > 2 {
                    continue;
                }
                num
            }
            Err(_) => continue,
        };

        loop {
            println!("\n\nType a number to convert\n\n");
            let mut input_number: String = String::new();
            io::stdin()
                .read_line(&mut input_number)
                .expect("Failed to read selection");

            let number_to_convert: f32 = match input_number.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!(
                "\nInput value is: {}, the result is {}",
                number_to_convert,
                temp_converter(number_to_convert, selected)
            );
            break 'end_program;
        }
    }
}

fn temp_converter(value: f32, options: i32) -> f32 {
    if options == 1 {
        return (value - 32.0) * (5.0 / 9.0);
    } else if options == 2 {
        return (value * (9.0 / 5.0)) + 32.0;
    };
    return 0.0;
}
