///  ============ 所有权 ====================


fn main(){
    let str = String::from("hello world");
    let (string,len) = get_str_len(str);
    println!("字符串：{string},长度：{len}");

}

fn get_str_len(string: String)->(String,usize){
    let len = string.len();
    (string,len)
}



// fn main(){
//
//     let s = gives_ownership(); //gives_ownership将返回值转给s
//
//     let s2 = String::from("hello"); // s2进入作用域
//
//     let s3 = takes_and_gives_back(s2); // s2被移动到函数中，并将返回值转移给s3，同时s2已失效。
//
//     println!("s:{s},s3:{s3}");
//
// }
//
//
// fn gives_ownership() -> String{
//     let str  = String::from("yous");
//     str
// }
//
// fn takes_and_gives_back(a_string: String) -> String{
//     a_string
// }

















// fn main() {
//     let s = String::from("hello");  // s进入作用域
//     takes_ownership(s);                  // s的值移动到函数里，如果后面还想用s 可以调用clone方法
//     // println!("s的值：{s}");               // 到这的时候s不再有效  println!会报错
//
//
//     let x = 10;                       // x进入作用域
//     makes_copy(x);                 //x移动到函数里，但i32是copy的，所以后面还可以用
//     println!("x的值：{x}");                // 不会报错
//
//
//
// }
//
// fn takes_ownership(str: String) { //str进入作用域
//     println!("字符串str:{str}");
// }// 这里 str移除作用域并调用drop方法，
// // 占用的内存释放
//
// fn makes_copy(number: i32){ //number进入作用域
//     println!("copy的值:{number}")
// }//这个一处number作用域，没做其他操作

// fn main(){
//     let s = String::from("hello");
//     let s2 = s.clone();
//     println!("s的值：{}",s);
//     println!("s2的值：{}",s2);
// }

// fn main(){
//     let s = String::from("hello");
//     let s2 = s;
//     println!("s的值：{}",s)
// }

// fn main(){
//     let x = 5;
//     let y = x;
//     println!("x的值，{} \ny的值：{}",x,y);  // 整数都有其固定的大小，所以x和y都被放入到了栈中
//
//     let s = String::from("hello");
//     let s2 = s;
//
//     // 而String类型不同，
//
// }

// fn main(){
//
//     {
//     let mut s = String::from("hello");
//     println!("s的值:{}",s);
//     // 修改字符串
//     s.push_str(" String");
//     println!("s的值:{}",s);
//
//     }
//
// }


// fn main(){
//     println!("============所有权==========");
//
//     {
//                                  // str 在这无效
//         let str = "hello"; // 从这开始 str有效
//                                 // str 有效
//     }
//     // 出作用域后 str 无效
// }