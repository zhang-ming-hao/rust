fn main()
{
    struct Color1(u8, u8, u8);

    let red = Color1(255, 0, 0);

    println!("{}, {}, {}", red.0, red.1, red.2);

    struct Color2 
    {
        r: u8,
        g: u8,
        b: u8
    }

    let blue = Color2{r:0, g:0, b:255};

    println!("{}, {}, {}", blue.r, blue.g, blue.b);

}