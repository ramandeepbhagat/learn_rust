// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person<'a> {
    name: &'a String,
    age: u8,
}

#[derive(Debug)]
// A unit struct
struct Unit;

#[derive(Debug)]
// A tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn rect_area(&self) -> f32 {
        ((&self.p1.x - &self.p2.x) * (&self.p1.y - &self.p2.y)).abs()
    }
}

fn square(point: Point, f: f32) -> Rectangle {
    Rectangle {
        p1: point,
        p2: Point { x: f, y: f },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name: &name, age };

    // Print debug struct
    println!("\n{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: left_edge, y: top_edge },
        p2: bottom_right,
    };

    println!("{:?}", rectangle);

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Area of rect = {}", rectangle.rect_area());

    let square: Rectangle = square(Point { x: 0.6, y: 0.6 }, 1.2);
    println!("square: {:?}", square);
}
