/*
 * RustのPublic,Private。
 * CreatedAt: 2019-06-18
 */
fn main() {
    println!("{}", my_mod::public());
//    println!("{}", my_mod::private()); // error[E0603]: function `private` is private
//    println!("{}", my_mod::my_mod_1::public()); // error[E0603]: module `my_mod_1` is private
}
mod my_mod {
    pub fn public() -> String { private(); String::from("my_mod::public()") }
    fn private() -> String { String::from("my_mod::private()") }
    mod my_mod_1 {
        pub fn public() -> String { super::private(); String::from("my_mod::my_mod_1::public()") }
    }
}
