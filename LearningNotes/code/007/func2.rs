fn main()
{
    let a = ("rust", "great");
    func1(a);
    func2(a);
    func3(a);
}

fn func1((name, how): (&str, &str))
{
    println!("{} {}", name, how);
}

fn func2((_, how): (&str, &str))
{
    println!("{}", how);
}

fn func3((name, _): (&str, &str))
{
    println!("{}", name);
}