use std::fs::File;
use std::io::Read;
use mysql::{Pool, PooledConn};
use serde_derive::Deserialize;

/// 数据库配置
/// 当属性作用于整个 crate 时，它们的语法为 #![derive]，当它们用于模块或项时，语法为 #[derive]（注意少了感叹号 !）
/// Deserialize  反序列化
/// Default： 创建数据类型的一个空实例
/// Debug：使用 {:?} formatter 来格式化一个值
///
///
#[allow(dead_code)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Database {
    host: String,
    port: usize,
    username: String,
    password: String,
    name: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Application {
    port: usize,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Setup {
    kick: bool,
}


#[allow(dead_code)]
#[derive(Deserialize)]
#[derive(Debug)]
pub struct Config {
    database: Database,
    application: Application,
    setup: Setup,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    id: u32,
    login_id: String,
    name: String,
    phone: String,
}


fn open_file(file_name: &str) -> File {
    match File::open(&file_name) {
        Ok(file) => file,
        Err(error) => panic!("打开文件错误 {} exception:{}", file_name, error)
    }
}


fn connection_mysql(db: &Database) -> PooledConn {
    let url = format!("mysql://{}:{}@{}:{}/{}", &db.username, &db.password, &db.host, &db.port, &db.name);
    println!("数据库连接url:{}", url);
    //创建数据库连接池
    let pool = Pool::new(&url).unwrap();
    match pool.get_conn() {
        Ok(connection) => connection,
        Err(e) => panic!("获取数据库链接错误:{e}")
    }
}


fn main() {
    let config_file_name = "config.toml";

    let mut file = open_file(&config_file_name);

    let mut str_val = String::new();

    match file.read_to_string(&mut str_val) {
        Ok(str) => str,
        Err(error) => panic!("读取文件失败：{error}")
    };

    let config: Config = toml::from_str(&str_val).unwrap();
    println!("config:{:?}", config);

    let database_config = config.database;
    let application_config = config.application;
    let setup_config = config.setup;

    println!("数据库配置：{:?}", database_config);
    println!("应用配置：{:?}", application_config);
    println!("设置配置：{:?}", setup_config);

    let mut connection = connection_mysql(&database_config);
    println!("数据库链接：{:?}", connection);

    let sql = "SELECT * FROM user";

    let data = connection.query(sql);
    let query_data = match data {
        Ok(d) => d,
        Err(e) => panic!("查询数据失败！{e}")
    };

    println!("数据：{:?}", query_data);
    let mut user_vector: Vec<User> = Vec::new();

    for row_result in query_data {
        match row_result {
            Ok(row) => {
                let user = User{
                    id: row.get("ID").unwrap(),
                    login_id: row.get("LOGINID").unwrap(),
                    name: row.get("NAME").unwrap(),
                    phone: row.get("PHONE").unwrap(),
                };

                user_vector.push(user);
            }
            Err(e) => panic!("查询数据失败：{e}")
        }
    }



    println!("用户集合{:?}", user_vector);

    println!("集合长度:{}",user_vector.len());
}






