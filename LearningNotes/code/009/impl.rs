struct Count
{
    x: i64,
    y: i64,
    z: i64
}

impl Count
{
    fn sum(&self) -> i64
    {
        self.x + self.y + self.z
    }

    fn x(&mut self, x: i64) -> &mut Count
    {
        self.x = x;
        self
    }
}

fn main()
{
    let mut a = Count{x: 10, y:14, z: 28};

    println!("{}", a.sum());
    println!("{}", a.setX(20).sum());

}