use std::cmp::Ordering;
use std::io;
use rand::Rng;


fn main(){
    println!("猜数字");
    println!("请输入你的数字");
    //生成随机数
    let gen_number = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("输入数字");
        let mut number  = String::new();
        io::stdin().read_line(&mut number).expect("读取失败");
        let number :u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        match number.cmp(&gen_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("对了");
                break;
            },
        }
    }
}

// fn main(){
//     println!("猜数字");
//     println!("请输入你的数字");
//
//     //生成随机数
//     let gen_number = rand::thread_rng().gen_range(1..=100);
//
//
//     let mut number  = String::new();
//     io::stdin().read_line(&mut number).expect("读取失败");
//     println!("你输入的数字是：{number}");
//
//     // 将输入的String类型转为u32类型
//     let number :u32 = number.trim().parse().expect("类型转换失败");
//     match number.cmp(&gen_number) {
//         Ordering::Less => println!("笑了"),
//         Ordering::Greater => println!("大了"),
//         Ordering::Equal => println!("对了"),
//     }
//
// }


// fn main() {
//
//     // 猜数字游戏
//     println!("猜数字游戏");
//     println!("请输入你的数字");
//
//     //定义变量  存储输入的数据   可变变量
//     let mut number = String::new();
//     // 定义变量的另一种方式  不可变变量
//     let number2 = String::new();
//     // 引入io库的写法
//     // io::stdin().read_line(&mut number).expect("读取失败");
//     //直接用库的写法
//     std::io::stdin()
//         .read_line(&mut number)
//         .expect("读取失败");  // expect 是 Result的方法
//     println!("你的数字是:{number}");
//     // number = "50".to_string();
//
//     // 改写法会报错  错误信息: cannot assign twice to immutable variable
//     // number2  = "100".to_string();
// }
