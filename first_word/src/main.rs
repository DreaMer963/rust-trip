fn main() {

    let s = String::from("hello world");
    let word = first_word(&s);
    let mut i = 0;
    while i<=word {
        print!("{}", s.chars().nth(i).unwrap());
        i = i+1;
    }
    println!();

    let res = first_word_(&s);
    println!("{}", res);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


// 使用字符串slice

fn first_word_ (s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
