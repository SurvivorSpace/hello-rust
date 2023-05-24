mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    hosting::add_to_wait_list();
}


// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
//
// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }
//

// mod back_of_house{
//     #![allow(dead_code)]
//     pub struct Breakfast{
//         pub toast: String,
//         seasonal_fruit: String
//     }
//
//     impl Breakfast{
//         pub fn summer(toast: &str) -> Breakfast{
//             Breakfast{
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches")
//             }
//         }
//     }
// }
//
// pub fn eat_at_restaurant(){
//     // 初始类型
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     //改变注意更改面包的类型
//     meal.toast = String::from("Wheat");
//     println!("类型：{}",meal.toast);
//
//    //  meal.seasonal_fruit = String::from("菠萝");  会报错
//
// }


//
// fn deliver_order(){}
//
//
// mod back_of_house{
//     pub fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
//
//     fn cook_order() {}
// }


// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_wait_list() {}
//
//         fn _seat_at_table() {}
//     }
//
//     mod serving {
//         fn _take_order() {}
//
//         fn _server_order() {}
//
//         fn _take_payment() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_wait_list();
//
//     //相对路径
//     front_of_house::hosting::add_to_wait_list();
// }