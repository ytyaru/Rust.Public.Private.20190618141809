/*
 * RustのPublic,Private。
 * CreatedAt: 2019-06-18
 */
fn main() {
    println!("{}", my_mod::public());
//    println!("{}", my_mod::private()); // error[E0603]: function `private` is private
}
mod my_mod {
    pub fn public() -> String { String::from("my_mod::public()") }
    fn private() -> String { String::from("my_mod::private()") }
}
