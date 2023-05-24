use std::fmt::format;

/// ============= 结构体 ========
///
///
// fn main() {
//     let rec = Rectangle { width: dbg!(30 * 2), height: 50 };
//     println!("宽：{}", rec.width);
//     println!("高：{}", rec.height);
//     dbg!(&rec);
// }
//
// #[derive(Debug)]
// struct Rectangle {
//     width: i32,
//     height: i32,
// }

// fn calculate_rectangle_area(rectangle: &Rectangle) -> i32 {
//     rectangle.width * rectangle.height
// }


// fn main() {
//     let  rec = Rectangle { width: 30, height: 50 };
//     let area = calculate_rectangle_area(&rec);
//     println!("长方形的面积：{}", area)
// }
//
// struct Rectangle {
//     width: i32,
//     height: i32,
// }
//
// fn calculate_rectangle_area(rectangle: &Rectangle) -> i32 {
//     rectangle.width * rectangle.height
// }

// fn main(){
//     let data = (30,50);
//     let area = calculate_area(data);
//     println!("长方形面积:{}",area)
// }
//
//
// fn calculate_area(data_tuple:(i32,i32)) -> i32{
//     data_tuple.0 * data_tuple.1
// }

// fn main(){
//     let width = 30;
//     let height = 50;
//
//     let area = calculate_area(width,height);
//     println!("长方形面积：{}",area)
// }
//
//
// fn calculate_area(width: i32,height: i32) -> i32{
//     width * height
// }


// 定义类单元结构体
// struct AlwaysEqual;
//
// fn main(){
//     let subject = AlwaysEqual;
// }

//fn main(){
//
//     let black = Color(0,0,0);
//     let origin = Point(0,0,0);zz
//
//     println!("color的值：{}",black.0);
//     println!("point的值：{}",origin.1)
//
// }
//
//
// #[allow(dead_code)]
// struct User {
//     id: u64,
//     name: String,
//     sex: bool,
//     email: String,
// }
//
// //定义元组结构体
// struct Color(i32,i32,i32);
// struct Point(i32,i32,i32);

// fn main() {
//
//
//     let user = User {
//         id: 10,
//         name: String::from("张三"),
//         sex: true,
//         email: String::from("12345678@qq.com"),
//     };
//
//     let user2 = User {
//         id: 11,
//         ..user
//     };
//
//
//     println!("用户的姓名：{}", user2.name);
//     println!("用户的姓名：{}", user.id);
// }


// fn build_user(name: String, sex: bool, email: String) -> User {
//     User {
//         id : 20,
//         name,
//         sex,
//         email,
//     }
// }

// fn build_user(id: u64, name: String, sex: bool, email: String) -> User {
//     User {
//         id,
//         name,
//         sex,
//         email,
//     }
// }


// fn build_user(id: u64, name: String, sex: bool, email: String) -> User {
//     User {
//         id: id,
//         name: name,
//         sex: sex,
//         email: email,
//     }
// }

// fn main() {
//     let mut user = User {
//         id: 10,
//         name: String::from("张三"),
//         sex: true,
//         email: String::from("12345678@qq.com"),
//     };
//
//     println!("用户的邮箱：{}", user.email);
//     user.email = String::from("test@163.com");
//     println!("用户的邮箱：{}", user.email);
//
//
// }




