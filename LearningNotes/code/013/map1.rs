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
    substr("abcd", 'c').map(|i| println!("result:{:?}", i));

    substr("abcd", 'e').map(|i| println!("result:{:?}", i));
}