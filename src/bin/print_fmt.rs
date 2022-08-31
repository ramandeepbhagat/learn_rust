

fn main() {
    let x = 5 + 15 + 80;
    let pi=3.141592;

    println!("\nIs `x` 10 or 100? x = {}", x);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    println!("Pi is roughly {pi:.*}", 3, pi=pi);
    println!("{}, `{pi:.*}` has 3 fractional digits", "Hi", 3, pi=pi);
    println!("{}, `{name:.*}` has 3 characters", "Hi", 3, name="Morning");
    println!("{}, `{name:->8.*}` has 3 right-aligned characters", "Hello", 3, name="1234.56");

}
