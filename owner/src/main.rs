fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("{:?}", slice);
}

// fn main() {
//     let my_string = String::from("hello world");
//     let word = first_word(&my_string[..]);

//     let my_string_literal = "hello world";
//     let word = first_word(my_string_literal);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let s = "hello world";

//     println!("{}", s);
// }

// fn main() {
//     let s = String::from("hello world");
//     let word = first_word(&s);

//     //s.clear(); //error
//     println!("{}", word);
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];

//     println!("hello = {}", hello); //hello = hello
//     println!("world = {}", world); //world = world
// }

// fn main() {
//     let s = String::from("hello world");
//     let word_index = first_world(&s);

//     println!("{}", word_index);
// }

// fn first_world(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let r = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello world");
//     &s
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let r1 = &s;
//     let r2 = &s;
//     let s1 = &mut s; //error

//     println!("{} {} {}", r1, r2, s1);
// }

// fn main() {
//     let mut s = String::from("hello world");
//     {
//         let s1 = &mut s;
//     }
//     let s2 = &mut s;
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let s1 = &mut s;
//     let s2 = &mut s; //error

//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// fn main() {
//     let mut s1 = String::from("hello world");

//     let len = calculate_length(&mut s1);

//     println!("'{}'的长度是{}", s1, len); //'hello world!!!'的长度是14
// }

// fn calculate_length(s: &mut String) -> usize {
//     s.push_str("!!!");
//     s.len()
// }

// fn main() {
//     let s1 = String::from("hello world");

//     let len = calculate_length(&s1);

//     println!("'{}'的长度是{}", s1, len); //'hello world'的长度是11
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s1 = String::from("hello world");

//     let (s2, len) = calculate_length(s1);

//     println!("'{}'的长度是{}", s2, len); //'hello world'的长度是11
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// fn main() {
//     let s1 = gives_ownership();

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("hello");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

// fn main() {
//     let s = String::from("hello world");

//     take_ownership(s);

//     let x = 10;

//     makes_copy(x);

//     println!("x = {}", x); // x = 10;
// }

// fn take_ownership(some_string: String) {
//     println!("{}", some_string); //hello world
// }

// fn makes_copy(some_number: i32) {
//     println!("{}", some_number); //10
// }

// fn main() {
//     let x = 10;
//     let y = x;

//     println!("x = {}", x); //x = 10
//     println!("y = {}", y); //y = 10
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("{}", s1); //hello
//     println!("{}", s2); //hello
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}", s1); //error
// }

// fn main() {
//     let mut s = String::from("Hello");

//     s.push_str(" String");

//     println!("{}", s); //Hello String
// }

// fn main() {
//     //s不可用
//     let s = "hello"; //s可用
//                            //可以对s进行相关操作
// } //s作用域到此结束，s不再可用