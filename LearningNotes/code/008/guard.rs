fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };

    match foo
    {
         Foo { x:(a, b), y} if a == 2 => println!("a = 2 b = {},  y = {} ", b, y),
         Foo { x:(a, b), y} if a == 1 => println!("a = 1 b = {},  y = {} ", b, y),
         //Foo { y: i, x: j } => println!("i = {:?}, j = {:?}", i, j),
         _ => println!("not match")
    }
}
