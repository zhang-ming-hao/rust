struct Count
{
    x: i64,
    y: i64,
    z: i64
}

impl Count
{
    fn new() -> Count
    {
        Count{x:0, y:0, z:0}
    }

    fn x(self, x: i64) -> Count
    {
        Count{x:x, y:self.y, z:self.z}
    }

    fn y(self, y: i64) -> Count
    {
        Count{x:self.x, y:y, z:self.z}
    }

    fn z(self, z: i64) -> Count
    {
        Count{x:self.x, y:self.y, z:z}
    }

    fn sum(&self) -> i64
    {
        self.x + self.y + self.z
    }
}

fn main()
{
    let a = Count::new()
                    .x(1)
                    .y(2);

    println!("{}", a.sum());
}