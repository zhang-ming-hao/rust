fn print(s: &str)
{
    match s.parse::<i32>()
    {
        Ok(n) => println!("{}", n),
        Err(e) => println!("{}", e),
    }
}

fn main()
{
    print("10");
    print("ttt");
}