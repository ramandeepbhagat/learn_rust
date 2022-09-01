use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [10, 20, 30, 40, 50];

    // Arrays are stack allocated
    // `size_of_val` returns the size of a variable in bytes
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1 .. 4]);

    for i in &xs[1 .. 4] {
        println!("{}", i);
    }

    for i in 0..xs.len() + 1 { // OOPS, one element too far
        match xs.get(i) {
            Some(val) => println!("Item at index {} = {}", i, val),
            None => println!("Slow down! Index {} is too far!", i),
        }
    }
}
