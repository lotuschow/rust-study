// 定义一个简单的声明宏
macro_rules! greet {
    () => {
        println!("Hello, world!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    greet!();           // 展开成 println!("Hello, world!");
    greet!("Alice");   // 展开成 println!("Hello, Alice!");
}
