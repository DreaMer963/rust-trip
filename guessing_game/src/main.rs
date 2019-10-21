use std::io;
use rand::Rng;  // Rng相当于一个trait
use std::cmp::Ordering;  // Ordering 是一个枚举

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101); // thread_rng()相当于随机数生成器
    println!("GUESS THE NUMBER");
    loop {
        println!("please input your guess");
        let mut guess = String::new();   // ::表明 new()是string类型的一个关联函数(针对类型实现的),就是静态方法
        io::stdin().read_line(&mut guess)  // 将返回一个值 Result(枚举类型)
            .expect("Failed to read guess");

        // 对guess进行转换
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input");
                continue;
            },
        };    //trim 方法会去除字符串开头和结尾的空白字符, parse()将字符串拆成数字

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Great");
                break;
            },
            Ordering::Greater => println!("Too big"),
        }
    }
}
