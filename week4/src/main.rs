use std::thread;
// use std::time::Duration;


fn max<T:PartialOrd>(a:T, b:T) -> T{
    if a > b {
        a
    } else {
        b
    }
}

trait Drawable {
    fn draw(&self, message:&str);
}

struct Circle{
    radius: f64
}

struct Square {
    side_length:f64
}

impl Drawable for Circle {
    fn draw(&self, message:&str) {
        println!("Drawing a Circle with radius {}: {}", self.radius, message);
    }
}

impl Drawable for Square {
    fn draw(&self, message:&str) {
        println!("Drawing a Square with side length {}: {}", self.side_length, message);
    }
}

fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn main() {
    //整数比较
    let max_int = max(5, 10);
    println!("The max int is {}", max_int);

    //浮点数比较
    let max_float = max(5.32, 10.38);
    println!("The max float is {}", max_float);

    // 创建 Circle 和 Square 的实例
    let circle = Circle { radius: 5.0 };
    let square = Square { side_length: 10.0 };

    // 调用它们的 draw 方法
    circle.draw("This is a circle");
    square.draw("This is a square");

    //比较字符串长度
    let string1 = String::from("long string is long");
    let sting2 = String::from("short string");

    let result = longest(string1.as_str(), sting2.as_str());
    println!("The longest string is {}", result);

    //多线程任务
    let handle1 = thread::spawn(||{
        //线程1的代码
        //比较字符串长度
        let string1 = String::from("long string is long");
        let string2 = String::from("short string");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {} from the thread-1", result);
    });

    let handle2 = thread::spawn(||{
        //线程1的代码
        //比较字符串长度
        let string1 = String::from("long string");
        let string2 = String::from("short string is short");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {} from the thread-2", result);
    });

    //等待两个线程执行完毕
    handle1.join().unwrap();
    handle2.join().unwrap();

    //主函数继续执行
    println!("主函数执行完毕");
}

