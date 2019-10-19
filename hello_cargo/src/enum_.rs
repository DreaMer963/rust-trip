pub(crate) enum Number {
    Int(i32),
    Float(f32),
}

pub(crate) fn read_num(num : &Number) {
    match num {
        &Number::Int(value) => println!("Integer {}", value),
        &Number::Float(value) => println!("Float {}", value),
    }
}