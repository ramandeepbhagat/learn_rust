fn main() {
    // Try changing the values in the array, or make it a slice!
    let array = [5, 3, 6, 9, 21, 11, 19];

    match array {
        // You can bind some and ignore the rest
        [5, .., last] => println!(
            "array[0] = -1, last = {} and all the other ones were ignored",
            last
        ),
        // You can bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // bind the second value, and store the rest of them in a single array
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}
