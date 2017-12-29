fn foo<'a, 'b:'a>(x: &'a str, y: &'b str) -> &'a str {
    if x > y {
        x
    } else {
        y
    }
}

fn main()
{
    let a = foo("abc", "abcd");

    println!("{}", a);
}