use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct User<'a> {
    name: &'a str,
    age: u8
}

impl fmt::Display for User<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "User name: {}, User age: {}", self.name, self.age)
    }
}

fn main() {
    let x = 5 + 15 + 80;
    println!("\nIs `x` 10 or 100? x = {}", x);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    println!("Pi is roughly {pi:.*}", 3, pi=3.141592);
    println!("{}, `{pi:.*}` has 3 fractional digits", "Hi", 3, pi=3.141592);
    println!("{}, `{name:.*}` has 3 characters", "Hi", 3, name="Morning");
    println!("{}, `{name:->8.*}` has 3 right-aligned characters", "Hello", 3, name="1234.56");

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let pikachoo: User = User {
        name: "Pikachoo",
        age: 20
    };
    println!("{:?}", pikachoo);
    println!("{:#?}", pikachoo);
    println!("{pikachoo}");
}
