#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let add = Operations::Add;
    let sub = Operations::Subtract;

    let sum = add.run(10, 20);
    println!("\nOperation: {:?}", add);
    println!("Sum: {:?}", sum);

    let diff = sub.run(10, 20);
    println!("\nOperation: {:?}", sub);
    println!("Diif: {:?}", diff);
}
