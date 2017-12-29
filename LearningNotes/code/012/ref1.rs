fn main()
{
    let mut a: String = String::from("xyz");
    a.push('a');
    
    {let b = &mut a;}

    a.push('b');
    println!("{}", a);
}