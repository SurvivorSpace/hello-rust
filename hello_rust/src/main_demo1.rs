
fn main(){
    println!("============复合类型==============");
    println!("=========数组============");
    let array = [1,2,3];
    let first = array[0];
    println!("数组的第一个元素:{first}");

    // 创建数组时申明类型和长度
    let array2: [i32;6] = [1,2,3,4,5,6];

    // 创建长度为5，元素默认值都是3的数组   = let array3 = [3,3,3,3,3];
    let array3 = [3,5];
}

// fn main(){
//     println!("============复合类型==============");
//     println!("=========元组============");
//     let tup:(i32,u8,f64) = (500,200,32.2);
//
//     let first_number = tup.0;
//     println!("元组的第一个元素:{first_number}");
//
//     let tup0 = (10,20.32.2);
//     let (x,y,z) = tup;
//
//     let tup2 = ();
//
//
// }


// fn main(){
//     println!("===========浮点型=========");
//     let a = 3.2;
//     let b: f32 = 3.2;
//
//     println!("===========布尔型===========");
//     let c = true;
//     let d = false;
//
//     println!("===========字符型===========");
//     let q = '陶';
//     let w = 'A';
// }



// fn main() {
//     println!("Hello Rust");
//     println!("======变量=========");
//     let number = 10;
//     println!("不可变变量number={number}");
//     let mut name = "张三";
//     println!("可变变量修改前name={name}");
//     name = "李四";
//     println!("可变变量修改后name={name}");
//
//     println!("=======常量========");
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     const NAME: &str  = "字符串常量";
//     println!("常量={THREE_HOURS_IN_SECONDS}");
//     println!("字符串常量={NAME}");
//
//     println!("===========隐藏===========");
//     let a = 10;
//     println!("申明a的值={a}");
//     let a = 20;
//     println!("隐藏后a的值={a}");
//     {
//         let a = a * 2;
//         println!("作用域内a的值={a}");
//     }
//     println!("作用域外a的值={a}");
//
//     println!("=========隐藏与mut的区别=========");
//     println!("-----隐藏");
//     let name = "hello rust";
//     println!("申明字符串变量name：{name}");
//     let name = name.len();
//     println!("隐藏字符串变量，获取字符串的长度={name}");
//     println!("-----mut");
//     //let mut nick_name   = "张三丰";
//     //nick_name = nick_name.len();
//
//     // println!("=========申明无符号的整数=======");
//     // let mut b:u32 = 2000;
//     // println!("无符号b的值：{b}");
//     // b = -2000;   //会报错   cannot apply unary operator `-`
//     // println!("修改无符号b的值为负数：{b}");
//
//     println!("=========申明有符号的整数=======");
//     let mut c:i32 = 2000;
//     println!("有符号c的值：{c}");
//     c = -2000;
//     println!("修改有符号c的值为负数：{c}");
//
//     // 256超出 u8的存储范围了
//    // let d:u8 = 256;
//
//     let s = 255u8;
//     println!("s的值:{s}");
//     let z = 1_000_000;
//     println!("z的值:{z}");
// }


