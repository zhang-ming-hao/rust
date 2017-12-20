fn main() 
{
    show(callback);
}

fn callback(a: i64)
{
    println!("{}", a);
}

fn show(func: fn(i64))
{
    func(10);
}