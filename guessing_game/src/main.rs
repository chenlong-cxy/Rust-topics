use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("欢迎来到猜数游戏!");
    //1、生成神秘数字
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字已经生成!");

    loop {
        //2、让用户进行猜测
        println!("请猜测:>");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        //3、将用户输入的数字字符串转化为整型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请您输入一个合法的整数!");
                continue;
            }
        };

        //4、将用户猜测的数与神秘数字进行比较
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("您猜测的数字太小了"),
            Ordering::Greater => println!("您猜测的数字太大了"),
            Ordering::Equal => {
                println!("恭喜您猜对了, 神秘数字就是{}!", secret_number);
                break;
            }
        }
    }
}

// use std::cmp::PartialEq;
// use std::cmp::Ordering;
// #[derive(Eq)]
// struct A {
//     a: i32,
// }
// impl A {
//     fn new(val: i32) -> A {
//         A { a: val }
//     }
// }
// impl PartialEq for A {
//     fn eq(&self, other: &A) -> bool {
//         self.a == other.a
//     }
//     fn ne(&self, other: &A) -> bool {
//         self.a != other.a
//     }
// }
// impl Ord for A {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.a.cmp(&other.a)
//     }
// }
// impl PartialOrd for A {
//     fn partial_cmp(&self, other: &A) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
// fn main() {
//     let a = A::new(10);
//     let b: A = A::new(20);
//     println!("{}" , a == b);
//     a.cmp(&b);
// }


// use rand::Rng;
// use std::io;

// fn main() {
//     println!("欢迎来到猜数游戏!");
//     //1、生成神秘数字
//     let secret_number = rand::thread_rng().gen_range(1, 101);
//     println!("神秘数字已经生成!");

//     loop {
//         //2、让用户进行猜测
//         println!("请猜测:>");
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("无法读取行");

//         //3、将用户输入的数字字符串转化为整型
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("请您输入一个合法的整数!");
//                 continue;
//             }
//         };

//         //4、将用户猜测的数与神秘数字进行比较
//         if guess < secret_number {
//             println!("您猜测的数字太小了");
//         } else if guess > secret_number {
//             println!("您猜测的数字太大了")
//         } else {
//             println!("恭喜您猜对了, 神秘数字就是{}!", secret_number);
//             break;
//         }
//     }
// }

// fn main() {
//     let a = 10;
//     let b = 20;
//     println!("ret: {}", a > b);
// }

// 进行比较猜测
// use std::cmp::Ordering;

// fn main() {
//     let secret_number = 10;
//     let guess = 20;

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("guess < secret_number"),
//         Ordering::Greater => println!("guess > secret_number"),
//         Ordering::Equal => println!("guess = secret_number"),
//     }
// }


// pub enum Result<T, E> {
//     /// Contains the success value
//     Ok(T),
//     /// Contains the error value
//     Err(E),
// }

// // 解析用户输入
// use std::io;

// fn main() {
//     loop {
//         println!("请输入一个整数:>");
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("无法读取行");
        
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("请您输入一个合法的整数!");
//                 continue;
//             }
//         };
//         println!("解析成功, 输入的数字是: {}", guess);
//     }
// }

// // 解析用户输入
// use std::io;

// fn main() {
//     println!("请输入一个整数:>");
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读取行");
    
//     let guess: u32 = guess.trim().parse().expect("请您输入一个合法的整数!");
//     println!("解析成功, 输入的数字是: {}", guess);
// }

// // 读取用户输入
// use std::io;

// fn main() {
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读取行");
//     println!("输入的内容: {}", guess)
// }

// 生成神秘数字
// use rand::Rng;

// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1, 101);
//     println!("生成的神秘数字是: {}", secret_number);
// }


//part4
// use rand::Rng;
// use std::io;
// use std::cmp::Ordering;

// fn main() {
//     println!("猜数游戏");
//     let secret_number = rand::thread_rng().gen_range(1, 101); //i32 u32 i64

//     loop {
//         println!("猜测一个数");
        
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("无法读取行");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("请输入一个整数");
//                 continue;
//             }
//         };

//         println!("你猜测的数是: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("太小了"), //arm
//             Ordering::Greater => println!("太大了"),
//             Ordering::Equal => {
//                 println!("猜对了");
//                 break;
//             }
//         }
//     }
// }

//part3
// use rand::Rng;
// use std::io;
// use std::cmp::Ordering;

// fn main() {
//     println!("猜数游戏");
//     let secret_number = rand::thread_rng().gen_range(1, 101); //i32 u32 i64
//     println!("神秘数字是: {}", secret_number);

//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读取行");
//     let guess: u32 = guess.trim().parse().expect("请输入一个整数");

//     println!("你猜测的数是: {}", guess);

//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("太小了"), //arm
//         Ordering::Greater => println!("太大了"),
//         Ordering::Equal => println!("猜对了"),
//     }
// }


//part2
// use rand::Rng;

// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1, 101);

//     println!("神秘数字是: {}", secret_number);
// }


//part1
// use std::io;

// fn main() {
//     println!("猜数!");
//     println!("猜测一个数");

//     // let mut foo = 1;
//     // let bar = foo; //immutable
//     // foo = 2;
//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess).expect("无法读取行");

//     println!("你猜测的数是: {}", guess);
// }
