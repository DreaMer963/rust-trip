fn main() {
    enum IpAddrKind{
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr{
        kind: four,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: six,
        address: String::from("::1"),
    };

    //另一种等效的方式
    enum IpAddr_{
        V4(String),
        V6(String),
    }
    let home = IpAddr_::V4(String::from("127.0.0.1"));
    let loopback = IpAddr_::V6(String::from("::1"));

    //
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // 枚举和结构体一样，都可以利用impl定义方法
    impl Message {
        fn call(&self) {

        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 枚举
    // rust 并没有空值NULL，不过它确实拥有一个可以编码存在或不存在概念的枚举。
    // 这个枚举是 Option<T>,标准库中的定义如下
    /*
    enum Option<T> {
        Some(T),
        None,
    }
    */

    let some_number = Option::Some(5); // 加不加Option::都行

    let absent_number: Option<i32>  = None; //需要指明类型

}
