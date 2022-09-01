fn iter_loop() {
    println!("\niter_loop");

    let names = vec!["Bob", "Frank", "Ferris"];

    /*
     iter - it borrows each element of the collection through each iteration.
     Thus leaving the collection untouched and available for reuse after the loop.
    */

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}

fn into_iter_loop() {
    println!("\ninto_iter_loop");

    let names = vec!["Bob", "Frank", "Ferris"];

    /*
    into_iter - it consumes the collection so that on each iteration the exact data is provided.
    Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    */

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}

fn iter_mut_loop() {
    println!("\niter_mut_loop");

    let mut names = vec!["Bob", "Frank", "Ferris"];

    /*
        iter_mut - it mutably borrows each element of the collection,
        allowing for the collection to be modified in place.
     */
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn main() {
    iter_loop();

    into_iter_loop();

    iter_mut_loop();
}
