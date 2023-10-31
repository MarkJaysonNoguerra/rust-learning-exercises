fn main() {
    // first example
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // second example refactoring with tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area_tuple(rect1)
    );

    // third example refactoring with struct

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_rectangle(&rect1)
    );
    // stdout
    println!("debug rect1 {:?}", rect1);
    // stderr
    dbg!(&rect1);

    // fourth recfactor struct with native method
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}

#[derive(Debug)] // to make rectangle debuggable
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area_using_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
