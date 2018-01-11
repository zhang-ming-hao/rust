extern crate rand;
use rand::Rng;

fn max(v: Vec<i32>) -> i32
{
    let mut max = -2147483648;

    for i in v
    {
        if i >max
        {
            max = i
        }
    }

    max
}

#[cfg(test)]
#[bench]
fn benchTest()
{
    let v: Vec<i32> = vec![];

    for i in 1 .. 1000
    {
        v.push(rand::thread_rng().gen_range(1, 101));
    }

    let m = max(v);

    println("{}", m);
}
