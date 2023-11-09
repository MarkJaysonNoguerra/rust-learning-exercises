use std::collections::{btree_map::Values, HashMap};

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("the scores is {:?}", scores);

    // accesing value from HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // Here, score will have the value that's associated with the Bue team and the
    // result will be 10. The get method returns an Option<&V>; if there's no value
    // for that key in the hashmap, get will return None. This program handles the
    // Option by calling copied to get an Option<i32> rather than an Option<&i32>, then
    // unwrap_or to set score to zero if scores doestn't have an entry for the key.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("the score for team blue is equal to {}", score);

    // Iterating over each key/value pair in hash map similar manner as we do with
    // vectors
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    // For types that implement the Copy trait, like i32, the values are copied into
    // the hash map. For owned values like String, the values will be moved in the hash map.
    // hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}, {}", field_name, field_value); // error

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding a Key and Value only if a Key Isn't Present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    // Another common use case for has maps is to look up key's value and then
    // update it based on the old value.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
