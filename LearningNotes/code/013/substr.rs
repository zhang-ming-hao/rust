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
    match substr("abcd", 'c') {
        Some(i) => println!("找到该字母，位于第{}个", i),
        None => println!("没有找到该字母"),
    }

    match substr("abcd", 'e') {
        Some(i) => println!("找到该字母，位于第{}个", i),
        None => println!("没有找到该字母"),
    }
}