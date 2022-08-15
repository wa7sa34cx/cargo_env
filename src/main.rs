// use env!("CARGO_PKG_NAME")::bar;
use std::env;

fn main() {
    println!("{}", env!("CARGO_PKG_NAME"));
}
