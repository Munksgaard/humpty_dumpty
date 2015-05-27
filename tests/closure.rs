#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(humpty_dumpty)]
#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::thread::spawn;

#[drop_protect]
struct Foo;

#[allow_drop="Foo"]
fn close(_: Foo) { }

#[test]
fn main() {
    let x = Foo;
    let guard = spawn(|| {
        close(x);
    });
    guard.join().unwrap();
}
