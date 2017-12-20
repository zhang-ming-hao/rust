fn main()
{
    let mut x = 0;
    output(&mut x);

    println!("{}", x);
}

fn output(x: &mut i64)
{
    println!("{}", x);
    *x = 10;
}