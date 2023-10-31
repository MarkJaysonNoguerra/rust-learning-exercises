fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let plus_one_none = plus_one(None);

    println!("Show the value of six {:?}", six);
    println!("Show the value of none {:?}", plus_one_none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
