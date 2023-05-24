#[derive(Debug)]
enum SpreadsheetCell{
    Width(i32),
    Height(i32),
    Text(String),
    IsTitle(bool)
}

// fn main(){
//
//     let mut vec  = Vec::new();
//     vec.push(SpreadsheetCell::Width(10));
//     vec.push(SpreadsheetCell::Height(20));
//     vec.push(SpreadsheetCell::Text(String::from("Vector存储枚举类型")));
//     vec.push(SpreadsheetCell::IsTitle(true));
//     println!("vec的第一个元素{:?}",&vec[0]);
//
// }



// fn main() {
//     let mut vec = vec![1, 2, 3, 4];
//     for i in &mut vec {
//         println!("元素:{}", *i);
//     }
// }
//

// fn main(){
//     let mut vec = vec![1, 2, 3, 4];
//
//     let first  = &vec[0];
//     println!("第一个数是：{}",first);
//
//     vec.push(6);
//
//    // println!("第一个数是：{}",first);  如果这段代码 放到这会报错  应为不能同时存在可变引用和不可变引用
//
// }

// fn main() {
//     let vec = vec![1, 2, 3, 4];
//
//     let index_0: &i32 = &vec[0];
//
//     println!("index_0={}", index_0);
//
//     let get_index_0 = vec.get(0);
//     match get_index_0 {
//         Some(get_index_0) => {
//             println!("get_index_0的值：{}", get_index_0);
//         }
//         None => {
//             println!("没有该值");
//         }
//     }
//
//     if let Some(item) = vec.get(5){
//         println!("index_get_0的值:{}",item);
//     }else{
//         println!("没有该值");
//     }
//
// }
