trait HasSum
{
    fn sum(&self) -> i64;
}

struct TwoSum
{
    x: i64,
    y: i64
}

impl HasSum for TwoSum
{
    fn sum(&self) -> i64
    {
        self.x + self.y
    }
}

struct ThreeSum
{
    x: i64,
    y: i64,
    z: i64
}

impl HasSum for ThreeSum
{
    fn sum(&self) -> i64
    {
        self.x + self.y + self.z
    }
}

fn getSum<T: HasSum>(obj: T)
{
    println!("sum = {}", obj.sum());
}

fn main()
{
    let a = TwoSum{x:1, y:2};
    getSum(a);

    let b = ThreeSum{x:1, y:2, z:3};
    getSum(b);
}