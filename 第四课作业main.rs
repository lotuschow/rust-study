// 定义三种不同的类型
struct TypeA;
struct TypeB;
struct TypeC;

// 为每个类型定义方法
impl TypeA {
    fn method_a(&self) {
        println!("Calling method_a for TypeA");
    }
}

impl TypeB {
    fn method_b(&self) {
        println!("Calling method_b for TypeB");
    }
}

impl TypeC {
    fn method_c(&self) {
        println!("Calling method_c for TypeC");
    }
}

// 定义一个 Trait 来实现不同类型的方法调用
trait CallMethod {
    fn call_method(&self);
}

impl CallMethod for TypeA {
    fn call_method(&self) {
        self.method_a();
    }
}

impl CallMethod for TypeB {
    fn call_method(&self) {
        self.method_b();
    }
}

impl CallMethod for TypeC {
    fn call_method(&self) {
        self.method_c();
    }
}

fn main() {
    // 创建一个存储 Trait Object 的 Vec
    let mut vec: Vec<Box<dyn CallMethod>> = Vec::new();
    vec.push(Box::new(TypeA));
    vec.push(Box::new(TypeB));
    vec.push(Box::new(TypeC));

    // 遍历 Vec 并调用方法
    for item in vec.iter() {
        item.call_method();
    }
}
