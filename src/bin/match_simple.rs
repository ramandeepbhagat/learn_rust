fn main() {
    let num = 27_i32;

    match num {
        1 => println!("One"),
        2..=5 => println!("todler"),
        6..=12 => println!("young"),
        13..=19 => println!("teen"),
        _ => print!("adult")
    }

    let is_admin = true;

    let result = match is_admin {
        false => 0,
        true => 1
    };

    println!("\nadmin: {}", result);
}
