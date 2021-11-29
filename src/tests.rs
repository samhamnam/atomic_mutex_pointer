use std::{
    ops::{Add, AddAssign},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use rand::Rng;

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
    Amp::try_lock(&a)
        .success(|a| a.add_assign(100))
        .error(|e| println!("Error: {}", e));
    println!("Amp: {:?}", a)
}

#[test]
fn funk() {
    let k = Amp::new(0);

    let mut t_list = vec![];
    for i in 0..1000 {
        let mut p = k.clone();
        t_list.push(
            thread::Builder::new()
                .name(format!("Thread: {}", i))
                .spawn(move || {
                    let mut rng = rand::thread_rng();
                    let i = rng.gen_range(100..2500);
                    thread::sleep(Duration::from_millis(i));
                    p.add_assign(i);
                    println!("{:?}", p);
                })
                .expect("whoop"),
        );
    }
    t_list.into_iter().for_each(|t| t.join().unwrap());
    println!("{:?}", k);
}
