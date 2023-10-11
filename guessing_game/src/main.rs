//part4
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("猜数游戏");
    let secret_number = rand::thread_rng().gen_range(1, 101); //i32 u32 i64

    loop {
        println!("猜测一个数");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个整数");
                continue;
            }
        };

        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"), //arm
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}

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
