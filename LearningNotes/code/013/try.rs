fn atoi(s: &str) -> Result<i32, std::num::ParseIntError>
{
    let n = try!(s.parse::<i32>());
    Ok(n)
}

fn main()
{
    println!("{:?}", atoi("10"));
    println!("{:?}", atoi("ttt"));
}