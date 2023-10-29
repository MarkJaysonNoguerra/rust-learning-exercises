fn main() {
    // Ownership Rules
    // 1. Each value in Rust has an owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // first example this example uses string literal which is a primative type
    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward

        println!("s is equal to {s}");
    }; // this scope is now over, and s is no longer valid

    // second example String Type
    let mut s = String::from("hello");

    s.push_str(", world!"); // pus_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // third example String Type
    {
        let s = String::from("hello"); // s is valid from this point forward

        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    // fourth example invalidated reference error
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world", s1);

    // forth example Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
