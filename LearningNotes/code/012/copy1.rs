#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64
}

fn main()
{
    let a = Point{x:10, y:20};
    let b = a;
    println!("a={:?} b={:?}", a, b);
}