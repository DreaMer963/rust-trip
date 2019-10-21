// 所有权（系统）是 Rust 最独特的功能，其令 Rust 无需垃圾回收（garbage collector）即可保障内存安全
// 通过所有权系统管理内存
// 所有权的存在就是为了管理堆数据

/*
所有权规则：
1. Rust中的每一个值，都有一个被称为其所有者的变量
2. 值有且只有一个所有者
3. 当所有者离开作用域，这个值被丢弃
*/

fn main() {
    //String 类型被存储在堆上,所以能够存储在编译时未知大小的文本
    let mut s = String::from("hello");
    s.push_str(", world");
    //对于 String 类型，为了支持一个可变，可增长的文本片段，
    // 需要在堆上分配一块在编译时未知大小的内存来存放内容。

    /* 需求：
    1. 必须在运行时向操作系统请求内存
    2. 需要一个当我们处理完 String 时将内存返回给操作系统的方法
    */
    // rust策略:内存在拥有它的变量离开作用域后就被自动释放(rust调用一个特殊的函数: drop)
    println!("{}", s);


    let x = 5;
    let _y = x; // 因为正数是有已知固定大小的简单值，所以这两个 5 被放入了栈中


    let s1 = String::from("ownership");
    let _s2 = s1;  // s1, s2 在栈中,他们都包含着指向堆上"hello"的指针(rust还多做了一些细节)，所以这不是"浅拷贝"
    //当s1和s2离开都离开作用域时
    //他们都会尝试释放相同的内存。这是一个叫做 二次释放（double free）的错误
    //那rust是怎么处理的?
    /*
    与其尝试拷贝被分配的内存，Rust 则认为 s1 不再有效，这个操作被称为 移动(move),所有权被移交
    因此 Rust 不需要在 s1 离开作用域后清理任何东西
    */
    //  println!("{}", s1)  error

    //"深拷贝", clone()
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);


    // 函数传值与上述类似

    //引用与借用
    // & 符号就是 引用，它们允许你使用值但不获取其所有权
    let s1 = String::from("hello");
    let len = foo(&s1);
    println!("{} length: {}", s1, len);

    //可变引用
    //可变引用的限制: 在特定作用域中的特定数据有且只有一个可变引用
    //也不能在拥有不可变引用的同时拥有可变引用
    //这个限制的好处是 Rust 可以在编译时就避免数据竞争

    change(&mut s);
    println!("{} length: {}", s1, len);  // 依然是: hello length: 5

    //悬垂引用
    /*
    在 Rust 中编译器确保引用永远也不会变成悬垂状态：
    当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
    */
}


fn foo(s: &String) -> usize {
    s.len()
}   // 这里不会发生drop

fn change(s: &mut String) {
    s.push_str(", world");
    println!("s: {}", s);
}
