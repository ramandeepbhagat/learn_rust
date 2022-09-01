use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "hello"; // &str
    let my_string = String::from(my_str); // String
    println!("my_string is {:?}", my_string);

    let num = Number::from(30);
    println!("num is {:?}", num);

    let int_num: i32 = 5;
    let another_num: Number = int_num.into();
    println!("another_num is {:?}", another_num);
}
