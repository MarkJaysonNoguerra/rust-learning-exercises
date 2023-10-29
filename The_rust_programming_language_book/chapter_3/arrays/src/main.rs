fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // You write an array’s type using square brackets with the type of each element,
    // a semicolon, and then the number of elements in the array, like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for each element by specifying the initial value,
    // followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let a = [3; 5];

    // array accesss
    let first_a = a[0];
    let second_a = a[1];

    println!("first a is: {first_a}, second a is: {second_a}");
}
