fn substr(string:&str, sub:char) -> Option<usize>
{
    for (i, c) in string.char_indices()
    {
        if c == sub
        {
            return Some(i);
        }
    }
    
    None
}

fn main()
{
    println!("{:?}", substr("abcd", 'c').unwrap_or(0));

    println!("{:?}", substr("abcd", 'e').unwrap_or(0));
}