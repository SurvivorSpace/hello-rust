
use crate::garden::vegetables::Asparagus;

//告诉编译器 应该在 src/garden文件中发现代码
pub mod garden;

/// main是根 crate
fn main() {
    let plant = Asparagus{};
    println!("plant:{:?}",plant)
}
