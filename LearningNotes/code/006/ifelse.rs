fn main()
{
    let a = 0;
    if a == 0
    {
        println!("right");
    }
    else {
        println!("error");
    }

    let mut b = "";
    if a== 0 
    {
        b = "right";
    }
    else 
    {
        b = "error";
    }
    println!("b={}", b);

    let c = 
        if a== 0 
        {
            "right"
        }
        else 
        {
            "error"
        };

    println!("c={}", c);
}