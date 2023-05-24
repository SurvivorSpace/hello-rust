
fn main(){
    for c in "Зд".chars() {
        println!("{c}");
    }

    for i in "hello".chars(){
        println!("{i}");
    }

    //返回字节
    for i in "hello".bytes(){
        println!("{i}");
    }

    for i in "你好".chars(){
        println!("{i}");
    }
}


// fn main(){
//     let hello = String::from("hello");
//     let world = String::from("world");
//     let s = format!("{hello} {world}");  //宏不会会获取所有权
//     println!(" s:{}", s);
//     println!("hello:{}",hello);
// }

// fn main() {
//     let hello = String::from("hello");
//     let world = String::from("world");
//     let world = String::from("world");
//     let str = hello + " " + &world;  // hello被移动了 不能在使用了
//
//     println!("str的值:{}", str);
//     let hello = String::from("hello");
// }


//
// fn main(){
//
//     let mut str = String::new();
//     str.push_str("hello world");
//     println!("str的值:{}",str);
//     let mut str = String::from("hello");
//     str.push_str(" rust");
//     str.push('。');
//     println!("str的值:{}",str);
//
// }


//
// fn main(){
//     //创建字符串
//     let mut str = String::new();
//     str.push('你');
//
//
//     let s = "hello world".to_string();
//     println!("字符串的值：{}",str);
//     println!("字符串的值：{}",s);
//
//     // 使用字符串字面值直接创建
//     let string = String::from("你好世界");
//     println!("Sting的值:{}",string);
//
//     let hello = String::from("السلام عليكم");
//     println!("hello：{}",hello);
//     let hello = String::from("Dobrý den");
//     println!("hello：{}",hello);
//     let hello = String::from("Hello");
//     println!("hello：{}",hello);
//     let hello = String::from("שָׁלוֹם");
//     println!("hello：{}",hello);
//     let hello = String::from("नमस्ते");
//     println!("hello：{}",hello);
//     let hello = String::from("こんにちは");
//     println!("hello：{}",hello);
//     let hello = String::from("안녕하세요");
//     println!("hello：{}",hello);
//     let hello = String::from("你好");
//     println!("hello：{}",hello);
//     let hello = String::from("Olá");
//     println!("hello：{}",hello);
//     let hello = String::from("Здравствуйте");
//     println!("hello：{}",hello);
//     let hello = String::from("Hola");
//     println!("hello：{}",hello);
//
// }