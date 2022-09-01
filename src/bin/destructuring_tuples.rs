fn main() {
    let triple = (9, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);

    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        // `..` can be used to ignore the rest of the tuple
        (9, ..)  => println!("First is `9` and the rest doesn't matter"),
        // `_` means don't bind the value to a variable
        _      => println!("It doesn't matter what they are"),
    }
}
