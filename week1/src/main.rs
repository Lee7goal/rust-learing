
fn varanddatatypes() {
    let x:i32 = 10;
    let y:f64 = 3.14;
    let name: &str = "lee";
    println!("x = {}", x);
    println!("y = {}", y);
    println!("name = {}", name);
}

fn controlflow() {
    let num:i32 = 5;
    if num > 10 {
        println!("Number is greater than 10")
    } else {
        println!("Number is less than or equal to 10")
    }

    let mut n:i32 = 0;
    let greeter: &str = "Hello, Rust";

    loop {
        if n > 5 {
            break;
        }
        println!("{}!", greeter);
        n+=1
    }

    let mut m: i32 = 1;
    while m <= 10 {
        println!("{}!", m);
        m += 1;
    }

    let nlist: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    for ele in nlist.iter() {
        println!("{}", ele)
    }
}



fn main() {
    varanddatatypes();
    controlflow();
}
