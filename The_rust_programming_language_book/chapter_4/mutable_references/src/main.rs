fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", word");
}
// NOTE: Mutable references have one big restriction
// If you have a mutable reference to a value, you cannot have other references to that value.
