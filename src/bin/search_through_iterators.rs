fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`.
    let mut iter = vec1.iter();
    // `into_iter()` for vecs yields `i32`.
    let mut into_iter = vec2.into_iter();

    // `iter()` for vecs yields `&i32`, and we want to reference one of its
    // items, so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vec1: {:?}", iter     .find(|&&x| x == 2));
    // `into_iter()` for vecs yields `i32`, and we want to reference one of
    // its items, so we have to destructure `&i32` to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("Find 2 in array1: {:?}", array1.iter()     .find(|&&x| x == 2));
    // `into_iter()` for arrays yields `i32`
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));

    /*
    Iterator::find gives you a reference to the item.
    But if you want the index of the item, use Iterator::position
    */

    // `iter()` for vecs yields `&i32` and `position()` does not take a reference, so
    // we have to destructure `&i32` to `i32`
    let index_of_first_even_number = vec1.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(1));

    match index_of_first_even_number {
        Some(n) => println!("Find index of 2 in vec1 using iter: {:?}", n),
        _ => ()
    }

    // `into_iter()` for vecs yields `i32` and `position()` does not take a reference, so
    // we do not have to destructure
    let index_of_first_negative_number = vec1.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);

    match index_of_first_negative_number {
        Some(n) => println!("Find index of (x < 0) in vec1 using into_iter: {:?}", n),
        _ => ()
    }
}
