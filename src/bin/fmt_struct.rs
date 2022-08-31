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

#[derive(Debug)]
struct Sum(i64, i64);

impl fmt::Display for Sum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let total = self.0 + self.1;
         write!(f, "{} + {} = {}", self.0, self.1, total)
    }
}

impl fmt::Binary for Sum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        let total = (self.0 + self.1) as f64;
        let magnitude = total.sqrt();
        let decimals = f.precision().unwrap_or(3);
        let string = format!("{:.*}", decimals, magnitude);
        f.pad_integral(true, "", &string)
        // write!(f, "{} + {} = {}", self.0, self.1, total)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{} + {}i", self.real, self.imag)
    }
}


fn main() {
    let sum = Sum(3, 4);
    let com = Complex { real: 3.3, imag: 7.2 };
    let pikachoo: User = User {
        name: "Pikachoo",
        age: 20
    };

    println!("\nDebug: {:?}", Structure(3));
    println!("Debug: {:?}", Deep(Structure(7)));

    println!("\nSum in Debug: {:?}", sum);
    println!("Sum in Display: {}", sum);
    println!("Sum in Binary: {:b}", sum);

    println!("\nCompare structures:");
    println!("Debug: {:?}", com);
    println!("Display: {}", com);

    println!("\nDebug: {:?}", pikachoo);
    println!("Pretty Debug: {:#?}", pikachoo);
    println!("Display: {pikachoo}");
}
