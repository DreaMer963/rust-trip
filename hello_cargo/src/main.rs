mod type_alias;
mod enum_;

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

    // 4. static (rust 唯一生成全局变量的方式)
    static G1 : i32 = 0; // 必须声明时初始化，且必须是编译期可以确定的常量, 可以调用const fn
    println!("{}", G1);

    static mut G2 : i32 = 5; // 可变全局变量，在读写时必须使用unsafe
    unsafe {
        G2 = 1;
        println!("{}", G2)
    }
    // println!("{}", G2) error

    // 5.const
    const C1 : i32 =  5;
    println!("{}", C1);

    // 6.char: 四字节, 描述一个unicode字符
    let _heart = '❤';

    let _x = b'A';  // 描述ASCII字符的方式
    let _x = b"hello";

    // 7.整数
    let _v1 = 10;
    let _v2 = 0x15;
    let _v3 = 0o7;    // 八进制
    let _v4 = 0b10;   // 二进制
    println!("9 power 3 = {}", 9_i32.pow(3));

    // 8.浮点数
    let _v5 = 5f32;
    let _v6 = 6f64;

    // 9.指针

    // 10.类型转换
    let v7 : i8 = 4;
    // let v8 : i16 = v7;
    let _v8 : i16 = v7 as i16; // 显示转换,合理使用，并不是任意类型转换都允许

    // 10.复合类型数据
    let t1 = (5, false);
    let _t2 = (5, (6, 7));
    let _t3 = (4,); // 元组只有一个元素的表达方式
    let _empty = ();  // 空元组
    println!("size of empty tuple {}", std::mem::size_of::<()>());
    let (_a, _b) = t1;
    let _a= t1.1;
    let _b = t1.0;

    // 11. struct
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }
    // 语法糖
    fn default() -> Point3D {
        Point3D { x: 1, y: 2, z:3 }
    }

    let point1 = Point3D{ x: 5, ..default()};
    println!("point1: y: {}, z: {}", point1.y, point1.z);

    let point2 = Point3D{ y:6, ..default()};
    println!("point2: x: {}, z: {}", point2.x, point2.z);

    // 12.tuple struct
    struct _Color (i32, i32, i32);

    // 13.enum
    let n: enum_::Number = enum_::Number::Int(10);
    enum_::read_num(&n);
}
