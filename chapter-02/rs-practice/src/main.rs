use core::api::perform_request;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io; // 使用标准库中的 io 这个模块

use core::product::Product;

mod core;

fn create_product() -> Product {
    let id = 1;
    let name = String::from("Product 1");
    let price = 100.0;

    Product::new(id, name, price)
}

fn main() {
    println!("Hello, world!");
    println!("Please input a number: ");
    let mut input = String::new(); // 在这里我们创建了一个新的 String，用来接收下面的输入
    io::stdin()
        .read_line(&mut input) // 读取一行
        .expect("Failed to read input!"); // 比较粗暴的错误处理
    println!("Your raw input is: {:?}.", input); // 打印输入的原始内容
    let number: usize = input.trim().parse().expect("Input is not a number!"); // trim 把前后的空格、换行符这些空白字符都去掉，parse 将输入的字符串解析为 i64 类型，如果解析失败就报错
    println!("Your input is: {}.", number); // 打印 parse 之后的 i64 数字

    // 如何打印出 PI 的前 n 位
    // let n = 10;
    // let mut s = String::new();
    // for i in 0..n {
    //     s.push_str(&(PI.to_string()[i..i + 1]));
    // }

    let mut product = create_product();
    println!("{:?}", product.id);
    println!("{:?}", product.name);
    println!("{:?}", product.price);

    product.add_cart(); // 加购

    product.receive_coupon(0.8); // 领券
    println!("{:?}", product.price);

    product.add_up(); // 凑单
    product.checkout(); // 结算

    let _: Vec<i32> = Vec::new();

    let mut v = Vec::new(); // 这里可以先不指定类型
    v.push(1);

    let v = vec!["Hello", "World"];
    for elem in &v {
        println!("{elem}");
    }

    println!("{:?}", v);

    let mut map: HashMap<String, i64> = HashMap::new();
    map.insert("a".to_string(), 2);

    match perform_request() {
        Ok(_) => println!("Request succeeded"),
        Err(e) => println!("Request failed: {}", e),
    }

    println!("{}", &(PI.to_string()[0..number + 1]));
}
