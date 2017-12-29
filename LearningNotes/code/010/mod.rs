mod m1
{
    const c:i64 = 10;

    pub fn print()
    {
        println!("我是m1模块的print函数");
    }

    pub mod m2
    {
        pub fn print()
        {
            println!("我是嵌套在m1模块中的m2模块的print函数");
        }
    }
}

fn main()
{
    m1::print();

    m1::m2::print();
}