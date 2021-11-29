use std::{
    ops::{Add, AddAssign},
    sync::{Arc, Mutex},
};

use crate::Amp;

#[test]
fn it_works() {
    let k = Arc::new(Mutex::new(5));
    if let Ok(mut v) = k.lock() {
        v.add_assign(1);
    }
    match k.lock() {
        Ok(mut v) => {
            v.add_assign(1);
        }
        Err(e) => {
            println!("Error lock unlocked: {}", e)
        }
    }
    if let Ok(mut v) = k.try_lock() {
        v.add_assign(1);
    }
    match k.try_lock() {
        Ok(mut v) => {
            v.add_assign(1);
        }
        Err(e) => {
            println!("Error lock unlocked: {}", e)
        }
    }
    println!("Arc<Mutex>: {:?}", k);

    let mut a = Amp::new(-5);
    a.add_assign(3);
    //a += 1;
    Amp::try_lock(&a);
    println!("Amp: {:?}", a)
}
