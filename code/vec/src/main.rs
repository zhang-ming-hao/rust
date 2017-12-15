fn main() 
{
    let v: Vec<i32> = vec![1, 2, 3, 4, 5]; 
    let i: usize = 0; 
    let j: i32 = 1; 
    let k = 2;

    // Works: 
    println!("{}", v[i]); 
    // Doesn’t: 
    println!("{}", v[j as usize]); 
    println!("{}", v[k]); 
}
