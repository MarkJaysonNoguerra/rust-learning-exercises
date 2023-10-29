fn main() {
    // first example
    // loop {
    //     println!("Again!");
    // }

    // second example
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // third example
    // third example loops with label useful with you have
    // innerloop that needs to break or continue the outer loop
    let mut count = 0;
    'continue_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("ramining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'continue_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // fourth example while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // fifth example
    // you can use while loop to loop through a collection like array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // sixth example
    // the best way to iterate over a collection
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // seventh example
    // countdown using for and range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    print!("LIFTOFF!!!");
}
