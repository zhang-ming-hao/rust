fn main()
{
    let mut sum = 0;

    for i in 1..100
    {
        sum += i;
    }
        
    println!("{}", sum);

    let a = [1,2,3,4,5];

    for x in a.iter()
    { 
        println!("{}", x); 
    }

    for (i, x) in a.iter().enumerate()
    { 
        println!("{}:{}", i, x); 
    }
}