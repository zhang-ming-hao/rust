extern crate projtest;

#[test]
fn test_0()
{
    let c = projtest::getcolor(0f64);

    assert!(c == 0f64);
}

#[test]
#[ignore]
fn test_1()
{
    let c = projtest::getcolor(1f64);

    assert!(c == 0f64);
}