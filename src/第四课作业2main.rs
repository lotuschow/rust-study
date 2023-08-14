// 定义一个自定义的类型
struct MyNumber(i32);

// 为 MyNumber 类型实现 Add trait
impl std::ops::Add for MyNumber {
    type Output = MyNumber;

    fn add(self, other: MyNumber) -> MyNumber {
        MyNumber(self.0 + other.0)
    }
}

// 定义一个 Trait 用于调用方法
trait PrintMethod {
    fn print(&self);
}

// 为 MyNumber 类型实现 PrintMethod Trait
impl PrintMethod for MyNumber {
    fn print(&self) {
        println!("Value: {}", self.0);
    }
}

fn main() {
    let num1 = MyNumber(5);
    let num2 = MyNumber(10);

    // 使用加法运算符进行加法计算
    let result = num1 + num2;
    result.print();
}
