#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(humpty_dumpty)]
#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(dead_code)]

#![deny(dropped_linear)]

use std::thread::spawn;

#[drop_protect]
struct Foo;

#[allow_drop="Foo"]
fn close(_: Foo) { }

fn main() {
    let x = Foo;
    let guard = spawn(|| {
        let y = x;
        //~^ERROR dropped var
        //~^^ERROR dropped var
        // FIXME: Not exactly sure why we need to here...
    });
    guard.join().unwrap();
}
