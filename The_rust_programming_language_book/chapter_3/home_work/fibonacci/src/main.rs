fn main() {
    println!("Hello, world!");
    println!("test fib: 5 result {}", fib(5));
    println!("test fib: 2 result {}", fib(2));
    println!("test fib: 6 result {}", fib(6));
    println!("test fib: 17 result {}", fib(17));
    println!("test fib: 23 result {}", fib(23));
}

fn fib(value: i32) -> i32 {
    if value < 2 {
        return value;
    }
    return fib(value - 1) + fib(value - 2);
}
