trait T1
{
    fn f1(&self);
}
trait T2
{
    fn f2(&self);
}

struct S1;

impl T1 for S1
{
    fn f1(&self)
    {
        println!("f1");
    }
}

impl T2 for S1
{
    fn f2(&self)
    {
        println!("f2");
    }
}

fn main()
{
    let s = S1{};
    s.f1();
    s.f2();
}