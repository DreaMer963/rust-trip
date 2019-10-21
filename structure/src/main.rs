fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // 元组结构体
    struct Color (i32, i32, i32);
    struct Point (i32, i32, i32);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 结构体方法，第一个参数总是self
        fn area(&self) -> u32 {
            self.width * self.height
        }
        // 关联函数
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }

    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let square1 = Rectangle::square(10);
    println!(
        "The area of the square is {} square pixels.",
        square1.area()
    );
}
