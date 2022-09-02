struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (2, 2), y: 5 };

    match foo {
        Foo { x: (1, b), y } => println!("\nFirst of x is 1, b = {}, y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 3, x: i } => println!("\ny is 3, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("\ny = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
}
