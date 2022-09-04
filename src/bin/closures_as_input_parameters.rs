/*
Fn: the closure uses the captured value by reference (&T)
FnMut: the closure uses the captured value by mutable reference (&mut T)
FnOnce: the closure uses the captured value by value (T)
*/

use std::mem;

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(fname: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    fname();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(fname: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    fname(3)
}

fn main() {

    let borrowed_str = "Hello!";
    let mut mutable_owned_str = "Good Bye".to_owned();

    let closure_fn = || {
        // `borrowed_str` is by reference: requires `Fn`.
        println!("I said {}.", borrowed_str);

        // Mutation forces `mutable_owned_str` to be captured by
        // mutable reference. Now requires `FnMut`.
        mutable_owned_str.push_str("!!!");
        println!("Then I screamed {}.", mutable_owned_str);

        // Manually calling drop forces `mutable_owned_str` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(mutable_owned_str);
    };

    // Call the function which applies the closure.
    apply(closure_fn);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled is {}", apply_to_3(double));
}
