include!(concat!(env!("OUT_DIR"),"/hello.rs"));

fn main() {
    println!(env!("OUT_DIR"));
    println!("{}",say_bye());
}
