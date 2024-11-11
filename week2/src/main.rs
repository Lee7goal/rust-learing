mod math;
mod string;


fn add(a:i32, b:i32) -> i32 {
    return a + b
}

fn is_even(a:i32) -> bool {
    if a % 2 == 0 {
        return true
    } else {
        return false
    }
}


fn main() {
    let ret = add(5, 6);
    println!("5和6的加和结果是: {:?}", ret);
    let s = 22;
    let res = is_even(s);
    if res {
        println!("{}是偶数", s)
    } else {
        println!("{}是奇数", s)
    }

    // 计算面积
    let num = 5;
    let squred = math::squre(num);
    println!("边长为{}的正方形面积为{}", num, squred);

    //反转字符串
    let wisper = "hello, rust!";
    let result = string::reverse(wisper);
    println!("{}反转后的字符串为{}", wisper, result);


    //借用以及所有权
    let own = "mtm is morn than world!";
    let other = &own;
    println!("own的值是{}", own);
    println!("other的值是{}", other);

}
    
