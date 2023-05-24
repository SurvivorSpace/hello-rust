/// ========== 引用============


fn main() {




}


// fn main(){
//     let mut s = String::from("hello world");
//
//     let string = get_first_world(&s);
//     println!("string的值:{string}")
//
// }
//
//
//  fn get_first_world(string: &String) -> &str{
//
//      //将字符串转为字节数组
//      let bytes_array = string.as_bytes();
//      // 通过iter()方法在数组上创建一个迭代器，iter方法返回数组中的每一个元素，
//      //enumerate包装了iter的结果，将这些元素作为元组的一部分来返回，返回的元组中，第一个元素是索引，第二个元素是数组中的引用。
//      for(index,&item) in bytes_array.iter().enumerate(){
//          if item == b' '{
//              return &string[..index];
//          }
//      }
//      string
//  }
// fn main(){
//     let  s = String::from("hello world");
//
//     let len = s.len();
//     let hello = &s[0..5];
//     //let hello = &s[..5];
//     let world = &s[6..11];
//     let world = &s[6..len];
//     let world = &s[6..];
//     println!("hello的值：{hello}");
//     println!("world的值：{world}");
//     let hello = &s[..5];
//
//     let all = &[0..s.len()];  //获取整个字符串的slice
//     let all = &[..];   //获取整个字符串的slice
// }

// fn main(){
//
//     let mut s = String::from("hello world");
//
//     let world = get_first_world(&s);
//
//     println!("第一个单词:{world}");
//     //清空了字符串  等于  ""
//     s.clear();
//     //world在这还是5
//     println!("第一个单词:{world}");
//
//
// }
//
// fn get_first_world(string: &String) -> usize{
//
//     //将字符串转为字节数组
//     let bytes_array = string.as_bytes();
//     // 通过iter()方法在数组上创建一个迭代器，iter方法返回数组中的每一个元素，
//     //enumerate包装了iter的结果，将这些元素作为元组的一部分来返回，返回的元组中，第一个元素是索引，第二个元素是数组中的引用。
//     for(index,&item) in bytes_array.iter().enumerate(){
//         if item == b' '{
//             return index;
//         }
//     }
//     string.len()
//
// }



//
// fn main(){
//     let mut s = String::from("hello");
//
//     let s1 = &s;
//     let s2 = &s;
//     println!("s1:{s1},s2:{s2}");
//     let s3 = &mut s;
//     println!("s3:{s3}");
//
// }



// fn main(){
//     let mut s = String::from("hello");
//     {
//         let s1 = &mut s;
//     }
//     let s2 = &mut s;
//
//
// }


// fn main(){
//     let mut s = String::from("你好 世界");
//     println!("s的值:{s}");
//     let s1 = &mut s;
//     let s2 = &mut s;
//     println!("s1:{s1},s2:{s2}");
// }


// fn main() {
//     let mut s = String::from("hello");
//     change_str(&mut s);
// }
//
// fn change_str(string: &mut String){
//     string.push_str(",world");
//     println!("string的值：{string}")
//
// }

// fn main(){
//
//     let s = String::from("hello world");
//
//     let len = get_str_len(&s);
//
//     println!("s的值:{s},长度:{len}");
//
// }
// fn get_str_len(string: &String)->(usize){
//     string.len()
// }
