fn main() {
    println!("Hello, world!");
    println!("DATABASE_URL: {}", std::env::var("DATABASE_URL").unwrap());
    // println!("local: {}", std::env::var("LOCAL").unwrap());
}
