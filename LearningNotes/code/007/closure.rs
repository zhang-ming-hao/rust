fn main()
{
    let c1 = | i:i32 | -> i32 {i+1};
    println!("{}", c1(3));

    let c2 = |i| i+2;
    println!("{}", c2(3));

    let color = "red";
    let print = || println!("Color: {}", color);

    print();
}