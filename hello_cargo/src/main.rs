mod type_alias;

fn main() {
    // 1. mut 读写和只读
    let s = "hello, world!";
    // rw
    let mut _sw = "hello, cargo";
    let (mut _a, mut _b) = (1, 2);

    let x: i32;   // 没有被初始化
    if  s=="" {
        x = 1;     // 初始话
        println!("{}", x)
    }

    //2. 变量遮掩
    let s = 5;
    println!("{}", s);  // 5

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v = v;
    for i in &v {
        println!("{}", i);
    }

    // 3.类型别名
    let x = 18;
    println!("20 years later: {}", type_alias::grow(x, 20));

}
