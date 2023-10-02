#![feature(decl_macro)]

make_struct!(Uh);

macro make_struct ($id:ident) {
    pub struct $id(pub i32);
}

fn main() {
    let k = Uh(2);
    println!("{}", k.0);
}

