fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// Using s as reference is called borrowing
// NOTE: You can't modify a value that you've borrowed
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope, But because it does not have ownership of
  // what it refers to, it is not dropped.
