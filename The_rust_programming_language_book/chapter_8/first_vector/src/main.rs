fn main() {
    // ways to define vectors
    let v: Vec<i32> = Vec::new();
    println!("value of v is {:?}", v);

    // defining vector using vec!
    let v = vec![1, 2, 3];
    println!("value of v is {:?}", v);

    // pushing/adding value to vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("value of v is equal to {:?}", v);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; this would cause an error as index 100
    // does not exist it's best to use the get method of vector that returns options
    let does_nt_exist = v.get(100);

    // example of borrow checker enforcing ownership and borrowing rules on vector
    // to ensure this reference any other references to the contents of the vector reamin valid
    // Recall that you can't have mutable and immutable references in the same scope
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6);

    println!("The first element is: {first}");
    // This might look like it should work; why should a reference to the first element care
    // about changes at the end of the vector? This error is due to the way vector works;
    // because vectors put the values next to each other in memory, adding a new element onto
    // the end of the vector might require allocating new memoery and copying the old elements to the new space,
    // if there isn't enough room to put all the elements next to each other where the vector
    // is currently stored. In that case the reference to the first element would be pointing to
    // deallocated memoery. The borrowing rules prevent programs from ending up in that situation

    // Iterating over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Iterating over the Values in a Vector and modifying its content
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("The new value of v is equal to {:?}", v);

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("the value of row is equal to {:?}", row);
}
