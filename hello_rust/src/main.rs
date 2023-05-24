use std::fs::File;
use std::io;
use std::io::Read;

fn main(){
    match  read_username_from_file(){
        Ok(username) => println!("读取成功：username={username}"),
        Err(error)=> panic!("读取失败!：{error}")
    }
}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(& mut username)?;
    Ok(username)
}


// use std::fs::File;
// use std::io;
// use std::io::Read;
//
// fn main(){
//     let username_result = read_username_from_file();
//     match username_result {
//         Ok(username) => println!("read file success.username={}",username),
//         Err(error) =>{
//            println!("读取文件失败:{}",error)
//         }
//     }
//
//     println!("程序继续执行......")
// }
//
//
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut username_file_result = File::open("hello")?;
//     let mut username = String::new();
//     username_file_result.read_to_string(&mut username)?;
//     Ok(username)
// }




// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io::{self, Read};
//
//     fn read_username_from_file() -> Result<String, io::Error> {
//         let username_file_result = File::open("hello.txt");
//
//         let mut username_file = match username_file_result {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };
//
//         let mut username = String::new();
//
//         match username_file.read_to_string(&mut username) {
//             Ok(_) => Ok(username),
//             Err(e) => Err(e),
//         }
//     }
// }



// fn main(){
//     // let file_result = File::open("hello.txt").unwrap();
//     // println!("{:?}",file_result);
//
//     let file_result = File::open("hello.txt").expect("项目中没有找到指定文件");
//     println!("{:?}",file_result);
//
//
// }
// fn main() {
//     let file_result = File::open("hello.txt");
//
//     match file_result {
//         Ok(file) => {
//             println!("打开文件成功");
//             file
//         },
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => {
//                     println!("创建文件");
//                     fc
//                 },
//                 Err(e) => panic!("文件创建失败：{:?}", e)
//             },
//             other_error => {
//                 panic!("打开文件失败：{:?}", other_error)
//             }
//         }
//     };
// }


// fn main() {
//     let file_result = File::open("hello.txt");
//
//     let file = match file_result {
//         Ok(file) => file,
//         Err(error) => panic!("打开文件失败：{:?}", error),
//     };
//
//     println!("{:?}",file);
// }


// fn main() {
//
//     panic!("程序出错！");
//
// }