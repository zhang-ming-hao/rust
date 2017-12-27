struct Count
{
    x: i64,
    y: i64,
    z: i64
}

impl Count
{
    fn new(x:i64, y:i64, z:i64) -> Count
    {
        Count{x:x, y:y, z:z}
    }

    fn sum(&self) -> i64
    {
        self.x + self.y + self.z
    }
}

fn main()
{
    let a = Count::new(1, 2, 3);

    println!("{}", a.sum());
}