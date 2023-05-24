use std::collections::HashMap;

fn main(){

    let text = "hello world wonderful world";

    let mut map  = HashMap::new();

    for word in text.split_whitespace(){
        println!("map:{:?}",map);
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map:{:?}",map);



}



//
// fn main(){
//     let mut map  = HashMap::new();
//     map.insert(String::from("team_blue"),10);
//     map.insert(String::from("team_yellow"),20);
//
//     // 如果不存在就添加这个key并关联这个值
//     map.entry(String::from("team_yellow")).or_insert(15);
//     let value  = map.entry(String::from("team_red")).or_insert(16);
//     println!("value:{}",*value);
//     println!("map:{:?}",map);
//
// }

//
// fn main(){
//     let mut map  = HashMap::new();
//     map.insert(String::from("blue"),10);
//     map.insert(String::from("blue"),20);
//     println!("{:?}",map);
// }
//




// fn main(){
//
//     let team_name = String::from("AR");
//
//     let mut hash_map = HashMap::new();
//     hash_map.insert(team_name,5);
//
//     // println!("{team_name}") //这里team_name就不能在用了
//
//
// }


// fn main() {
//     let mut hash_map = HashMap::new();
//     hash_map.insert(String::from("team_blue"), 10);
//     hash_map.insert(String::from("team_yellow"), 100);
//
//     let score = hash_map.get(&String::from("team_blue")).copied().unwrap_or(0);
//     println!("team_blue={score}");
//
//     // 以任意顺序打印
//     for (key, value) in &hash_map {
//         println!("key：{key} value：{value}");
//     }
// }

