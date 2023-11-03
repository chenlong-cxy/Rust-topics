// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     println!("hello world");

//     let user = User {
//         email: "someone@example.com",
//         username: "dragon",
//         active: true,
//         sign_in_count: 10,
//     };
// }

// #[derive(Debug)]
// struct Color(i32, i32, i32);
// #[derive(Debug)]
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);

//     println!("{:?}", black);
//     println!("{:?}", origin);
// }

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         username: String::from("dragon"),
//         email: String::from("dragon@examp.com"),
//         sign_in_count: 1,
//         active: true,
//     };

//     let user2 = User {
//         username: String::from("lee"),
//         email: String::from("lee@examp.com"),
//         ..user1
//     };

//     println!("{:?}", user1);
//     println!("{:?}", user2);
// }

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let user1 = User {
//         username: String::from("dragon"),
//         email: String::from("dragon@examp.com"),
//         sign_in_count: 1,
//         active: true,
//     };

//     let user2 = User {
//         username: String::from("lee"),
//         email: String::from("lee@examp.com"),
//         sign_in_count: user1.sign_in_count,
//         active: user1.active,
//     };

//     println!("{:?}", user1);
//     println!("{:?}", user2);
// }

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     println!("hello world");

//     let user = build_user(String::from("someone@examp.com"), String::from("dragon"));

//     println!("{:?}", user);
// }

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     println!("hello world");

//     let user = build_user(String::from("someone@examp.com"), String::from("dragon"));

//     println!("{:?}", user);
// }

// #[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     println!("hello world");

//     let mut user = User {
//         email: String::from("someone@example.com"),
//         username: String::from("dragon"),
//         active: true,
//         sign_in_count: 10,
//     };

//     user.email = String::from("another@examp.com");

//     println!("{:?}", user);
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     println!("hello world");

//     let user = User {
//         email: String::from("someone@example.com"),
//         username: String::from("dragon"),
//         active: true,
//         sign_in_count: 10,
//     };
// }
