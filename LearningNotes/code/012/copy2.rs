#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}
impl Copy for Point{}
impl Clone for Point
{
    fn clone(&self) -> Point
    {
        Point{x:self.x, y:self.y}
    }
}

fn main()
{
    let a = Point{x:10, y:20};
    let b = a;
    println!("a={:?} b={:?}", a, b);
}