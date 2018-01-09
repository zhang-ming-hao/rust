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
    substr("abcd", 'c').and_then(|i| Some(i));

    substr("abcd", 'e').and_then(|i| Some(i));
}