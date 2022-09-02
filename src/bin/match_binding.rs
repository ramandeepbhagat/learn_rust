fn some_number() -> Option<u32> {
    Some(12)
}

fn main() {

    match some_number() {
        Some(x) if x <= 10 => println!("0 to 10 num = {x}"),
        Some(x @ 11..=20) => println!("11 to 20 num = {x}"),
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n)      => println!("Not interesting... {}", n),
        // `None` variant
        // None => panic!(),
        // all other numbers
        _ => panic!(),
    }
}
