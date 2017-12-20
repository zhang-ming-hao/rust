fn main()
{
    let mut sum = 0;
    loop
    {
        if sum < 10
        {
            sum += 1;
        }
        else 
        {
            break;
        }
    }

    println!("{}", sum);
}