

fn main(){
    let number = five();
    println!("函数返回值：{number}");

    let gen = get_gen(10,200);
    println!("表达式返回值:{gen}");

    let c = get_gen_number(10,20);
    let d = get_gen_number2(10,20);
    println!("c的值={c}");
    println!("d的值={d}");
}

fn five() -> i32{
    5
}

fn get_gen(a: u32,b: u32) -> u32{
    a + b
}


fn get_gen_number(a: u32,b: u32) -> u32{
    let gen_number = a + b;
    gen_number
}

fn get_gen_number2(a: u32,b: u32) -> u32{
    let gen_number = a + b;
   return gen_number;
}

// fn main() {
//     println!("=======函数==========");
//     print_number(100);
//     print_str("张三",18);
//
//     let x = {
//         let y = 3;
//         y + 1;
//     };
//
//
// }
//
//
//
//
// fn print_number(number: u8){
//     println!("number的值是:{number}")
// }
//
// fn print_str(name: &str,age: u8){
//     println!("姓名是：{name}");
//     println!("年龄是：{age}");
// }


// fn author_function(){
//     println!("申明其他函数")
// }

