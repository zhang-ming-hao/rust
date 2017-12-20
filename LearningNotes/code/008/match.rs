fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };

    match foo
    {
         Foo { x:(2, b), y} => println!("b = {},  y = {} ", b, y),
         Foo { y: i, x: j } => println!("i = {:?}, j = {:?}", i, j),
         Foo { y, .. } => println!("y = {}", y),
    }
}

