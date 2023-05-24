#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn calculate_rectangle_area(&self) -> i32{
        self.width * self.height
    }

    fn width(&self) -> bool{
        self.width > 0
    }

}

fn main(){
    let rec = Rectangle{width:20,height:30};
    dbg!(&rec);
    println!("长方形面积：{}",rec.calculate_rectangle_area());
    println!("width方法:{}",rec.width())

}



