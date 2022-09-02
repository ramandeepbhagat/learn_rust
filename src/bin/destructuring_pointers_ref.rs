fn main() {
    /*
        Dereferencing uses *
        Destructuring uses &, ref, and ref mut
    */

    // Assign a reference of type `i32`.
    // The `&` signifies there is a reference being assigned.
    let reference = &4;

    match reference {
        &val => println!("\nGot a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // Accordingly, by defining 2 values without references,
    // references can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            // Got a reference.
            // Gotta dereference it before we can add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}
