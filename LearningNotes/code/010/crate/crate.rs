// 这个 crate 是一个二进制可执行文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "libcrate"]

pub fn print()
{
    println!("我是crate中的函数");
}