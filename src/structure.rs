struct Color{
    red: u8,
    green: u8,
    blue: u8
}

pub(crate) fn my_color(){
    let black = Color {red: 0, green: 0, blue: 0};

    println!("my_color");

    println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue);
}