//! # 函数getcolor
//! 将输入数据按照级别映射到0~255的颜色值
//! 调用示例如下：  
//! ```
//! let c = getcolor(0)
//! assert_eq!(c, 0)
//! ```


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