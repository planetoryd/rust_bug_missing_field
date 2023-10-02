#![feature(decl_macro)]

pub struct Uh(pub i32);

fn main() {
    let k = Uh(2);
    println!("{}", k.0);
}

