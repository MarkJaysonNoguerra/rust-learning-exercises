fn main() {
    // Different ways to defining String define take not there is a difference
    // between string slice(&str) vs String type
    let mut s = String::new(); // creating a string type

    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // Remember that strings are UTF-8 encoded, so we can inclde any properly
    // encoded data in them
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Appending to a String with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("the value of s is equal to {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push method
    let mut s = String::from("lo");
    s.push('l');
    println!("the value of is equal to {}", s);

    // Concantenation with the + Operator or the format! Macro
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // println!(
                       //     "the s1 is no longer valid as the value of it is now owned by s3 {}",
                       //     s1
                       // ); // error
    println!("The s3 is equal to {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s);

    // Bytes and Scalar Values and Grapheme Clusters
    // Another point about UTF-8 is that there are actually three relevant ways to look
    // at strings from Rust's perspective: as bytes, scalar values, and
    // grapheme clusters (the closest thing to waht we would call letters).

    // Methods for iterating Over Strings
    for c in "Зд".chars() {
        // iterate by chars which is scalar values
        println!("{c}");
    }

    for b in "Зд".bytes() {
        // by string bytes
        println!("{b}");
    }
}
