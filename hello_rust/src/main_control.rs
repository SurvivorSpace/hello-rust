fn main(){
    let array:[u8;5] = [1,2,3,4,5];
    for ele in array {
        println!("元素：{}",ele);

    }


    for number in (1..4).rev(){
        println!("number:{}",number);
    }
    println!("OFF")
}

// fn main(){
//     let mut count = 0;
//     while count < 100 {
//         count += 1;
//         println!("循环{count}次");
//     }
//
//     let array:[u8;5] = [1,2,3,4,5];
//     let mut index = 0;
//     while index < 5 {
//         println!("值:{}",array[index]);
//         index += 1;
//
//     }
//
// }

// fn main() {
//     let mut count = 0;
//
//     'loop_sign: loop {
//         let mut sign = 10;
//         loop {
//             if sign == 9 {
//                 break;
//             } else if count == 2 {
//                 println!("你好世界，第二次循环：{count}");
//                 break 'loop_sign;
//             }
//             sign -= 1;
//         }
//         count += 1;
//         println!("循环次数：{count}");
//     }
//
//     println!("count次数：{count}");
// }


//
//
// fn main(){
//     let mut number = 1;
//     loop {
//         println!("你好世界：{number}次");
//         number += 1;
//         if number == 100 {
//             break;
//         }
//     }
//     println!("number的值：{number}");
//
//
//
//     // 循环返回值
//     let num = loop{
//         println!("你好 rust");
//
//         if number == 101 {
//             break number * 2;
//         }else{
//             number += 1;
//         }
//
//     };
//
//     println!("num的值：{num}");
//
//
// }


// fn main() {
//
//     let bool = true;
//     let number = if bool {5} else {6};
//     println!("number的值:{number}")
//
// }


// fn main() {
//     println!("=============控制流====================");
//     let number = 5;
//     if number > 5 {
//         println!("number大于5");
//     } else if number == 5 {
//         println!("number等于5");
//     } else {
//         println!("number小于5");
//     }
// }
