fn simple_loop() {

    let mut count = 0u32;

    println!("\nLet's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;

            // println!("skipped");
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

fn nested_loop() {
    'outer: loop {
        println!("\nEntered the outer loop");

        #[allow(unused_labels)]
        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        // This point will never be reached
        // println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("\nResult from loop: {}", result)
}

fn while_loop() {
    println!("\nwhile_loop");

     // A counter variable
    let mut n = 1;

    // Loop while `n` is less than 101
    while n < 15 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }
}

fn for_loop() {
    println!("\nfor_loop_1");

    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    println!("\nfor_loop_2");
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..=11 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

fn main() {

    simple_loop();

    nested_loop();

    return_from_loop();

    while_loop();

    for_loop();
}
