#![allow(dead_code)]
pub struct Request{
    id: i32,
    url: String
}

impl Request {
    pub fn get_name(url: String) ->  String{
        let s = String::from("https");
        let new_url = format!("{s}://{url}");
        return new_url;
    }
}

fn main(){
    let new_url = crate::Request::get_name(String::from("www.baidu.com"));
    println!("url:{}",new_url);
}
