use std::io;

fn main()
{
    let mut buff = String::new();
    let mut name = String::new();
    let mut age = 0;

    'outer: loop
    {
        println!("请选择操作：");
        println!("1：显示信息");
        println!("2：修改信息");
        println!("3: 退出");
        std::io::stdin().read_line(&mut buff).unwrap();
        let choice: usize = buff.trim().parse().expect("Please type a number!");
        
        buff.clear();
        match choice 
        {
            1 => println!("name={}, age={}", name, age),
            2 => 
            {
                'inner: loop
                {
                    println!("请选择操作：");
                    println!("1：修改姓名");
                    println!("2：修改年龄");
                    println!("3: 保存");
                    println!("4: 退出");

                    std::io::stdin().read_line(&mut buff).unwrap();
                    let choice: usize = buff.trim().parse().expect("Please type a number!");
                    
                    buff.clear();
                    match choice
                    {
                        1 => 
                        {
                            print!("请输入姓名：");
                            std::io::stdin().read_line(&mut buff).unwrap();
                            name = buff.trim().to_string();
                            buff.clear();
                        },
                        2 =>
                        {
                            print!("请输入年龄：");
                            std::io::stdin().read_line(&mut buff).unwrap();
                            std::io::stdin().read_line(&mut name).unwrap();
                            age = buff.trim().parse().expect("Please type a number!");
                            buff.clear();
                        },
                        3 => break,
                        4 => break 'outer,
                        _ => continue
                    }
                }
            },
            3 => break,
            _ => continue,
        }
    }
}