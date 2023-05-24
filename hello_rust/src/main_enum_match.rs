
fn main(){
    let config_max  = Some(3u8);
    match config_max {
        Some(max) => println!("max={}",max),
        _ => ()
    }

    // if let 语法
    if let Some(max) = config_max {
        println!("max={}",max);
    }else{
        println!("max")
    }

}



//
//
// fn add_fancy_hat(){
//     println!("获得了一精致顶帽子");
// }
//
// fn remove_fancy_hat(){
//     println!("失去了精致的帽子");
// }
//
//
// fn move_player(num_space: i32){
//     println!("移动了{}步",num_space)
// }
//
//
//
// fn main(){
//     let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => move_player(dice_roll)
//     }
// }


//
// fn plus_one(x: Option<i32>) -> Option<i32>{
//     match x {
//         None => None,
//         Some(i) => Some(i + 1)
//     }
// }
//
//
// fn main(){
//
//     let five = Some(5);
//     let six = plus_one(five);
//     let none =  plus_one(None);
//
//     println!("five:{:?}",five);
//     println!("five:{:?}",six);
//     println!("five:{:?}",none);
//
// }


// #![allow(dead_code)]
// enum Coin {
//     Penny,
//     //便士
//     Nickel,
//     // 镍币
//     Dime,
//     // 一角硬币
//     Quarter(UsState),
// }
//
// #[derive(Debug)]
// enum UsState{
//     Alabama,
//     Alaska
// }
//
//
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("便士");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("from {:?}",state);
//             25
//         },
//     }
// }
//
// fn main() {
//     let value = value_in_cents(Coin::Quarter(UsState::Alabama));
//     println!("value:{value}")
// }
//
// //
// //
// // fn main(){
// //     let _some_bool = Option::Some(true);
// //     let some_number = Some(5);
// //     let _some_string = Some(String::from("option"));
// //     let _some_char = Some('a');
// //     let option_var: Option<String> = None;
// //     let number_bool = some_number.is_some();
// //     let number_bool_none = some_number.is_none();
// //
// //     println!("some_number{:?}",{some_number});  // some_numberSome(5)
// //     println!("number_bool是否有值：{number_bool}"); //true
// //     println!("number_bool是否为空：{number_bool_none}"); //false
// //     println!("option_var是否为空:{:?}",option_var.is_none()); //true
// //
// // }
// //
//
// // #[allow(dead_code)]
// // #[derive(Debug)]
// // enum Message {
// //     Quit,
// //     Move { x: i32, y: i32 },
// //     Write(String),
// //     ChangeColor(i32,i32,i32)
// // }
// //
// // impl Message {
// //     fn call(&self){
// //         // 方法体
// //         println!("self的值:{:?}",self)
// //     }
// //
// // }
// //
// //
// //
// // fn main() {
// //     let write = Message::Write(String::from("你好,世界"));
// //     println!("write:{:?}",write);
// //     write.call();
// //
// //
// //
// // }
//
//
// // struct Quit;
// // struct Move{
// //     x:i32,
// //     y:i32
// // }
// // struct Write(String);
// // struct ChangeColor(i32,i32,i32);
//
// //
// //
// // #[derive(Debug)]
// // enum IpAddr {
// //     IPV4(u8,u8,u8,u8),
// //     IPV6(String),
// // }
// //
// //
// // fn main() {
// //     let home = IpAddr::IPV4(127,0,0,1);
// //     let loopback = IpAddr::IPV6(String::from("::1"));
// //
// //
// // }
//
//
// // #[derive(Debug)]
// // enum IpAddr {
// //     IPV4(String),
// //     IPV6(String),
// // }
// //
// //
// // fn main() {
// //     let home = IpAddr::IPV4(String::from("127.0.0.1"));
// //     let loopback = IpAddr::IPV6(String::from("::1"));
// //     println!("home:{:?}", home);
// //     println!("loopback:{:?}", loopback);
// // }
//
//
// // fn main(){
// //     // 创建不同的实例
// //     let four  = IpAddrKind::IPV4;
// //     let six = IpAddrKind::IPV6;
// //     println!("IPV4:{:?}",four);
// //     println!("IPV4:{:?}",six);
// //
// //
// //
// // }