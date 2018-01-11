pub mod color
{
    pub fn getcolor(v:f64) -> f64
    {
        let k1 = 100.0 / 5.0;

        let k2 = (255.0 - 100.0) * 1.0 / (10.0 - 5.0);
        let b2 = k2 * 5.0 - 100.0;

        match v {
            n if n >=0f64 && n < 5f64 => return n * k1,
            n if n > 5f64 => return n * k2 + b2,
            _ => return -1f64,
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::color;

    #[test]
    fn test_0()
    {
        let c = color::getcolor(0f64);

        assert!(c == 0f64);
    }

    #[test]
    #[ignore]
    fn test_1()
    {
        let c = color::getcolor(1f64);

        assert!(c == 0f64);
    }
}
