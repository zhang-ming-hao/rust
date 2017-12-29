struct Person<'a>
{
    age: &'a u8
}

impl<'a> Person<'a>
{
    fn GetAge(&self) -> &u8
    {
        self.age
    }
}

fn main()
{
    let x = 30u8;
    let a = Person{age:&x};

    println!("{}", a.GetAge());
}