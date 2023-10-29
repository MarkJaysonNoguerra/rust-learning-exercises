fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {x}");
    println!("The value of y is: {y}");
    println!("The value of y is: {z}");

    // tuple access
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("tup.0 is: {five_hundred}, tup.1 is: {six_point_four}, tup.2 is: {one}");
}
