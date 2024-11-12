struct Person {
    name: String,
    age: u32,
    email: String,
}

enum Color {
    Red,
    Green,
    Blue
}

fn print_color(color:Color) {
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
    }
}

impl Person {
    fn new_person(name:String, age:u32, email:String) -> Person {
        Person{ name, age, email}
    }
}

fn describe_number(num: i32) {
    match num {
        0 => println!("Zero"),
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }
}

// 定义 divide 函数，返回 Result 类型
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(a / b)
    }
}



fn main() {
    let person = Person::new_person(String::from("Alice"), 30, String::from("alice@example.com"));

    //打印person字段
    println!("Name is: {}", person.name);
    println!("Age is: {}", person.age);
    println!("Email is: {}", person.email);

    //颜色
    print_color(Color::Red);
    print_color(Color::Green);
    print_color(Color::Blue);

    //数字匹配
    describe_number(0);
    describe_number(1);
    describe_number(2);
    describe_number(3);
    describe_number(-1);

    //处理错误
    // 调用 divide 函数并处理可能的错误
    let result1 = divide(10.0, 2.0);
    match result1 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("{}", e),
    }

    let result2 = divide(10.0, 0.0);
    match result2 {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("{}", e),
    }
}
