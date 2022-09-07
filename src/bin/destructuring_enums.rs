enum Color {
    // Red is specified solely by its name.
    #[allow(dead_code)]
    Red,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),

    #[allow(dead_code)]
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("\nWhat color is it?");
    // An `enum` can be destructured using a `match`.

    match color {
        Color::Red   => println!("The color is Red!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }

}
