use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct Amp<T> {
    v: Arc<Mutex<T>>,
}
impl<T> Amp<T> {
    pub fn new(t: T) -> Self {
        Self {
            v: Arc::new(Mutex::new(t)),
        }
    }
    pub fn try_lock(s: &Self) -> AmpContainer<T> {
        match s.v.try_lock() {
            Ok(mut v) => AmpContainer { v: Some(v) },
            Err(e) => todo!(),
        }
    }
}

pub struct AmpContainer<'l, T: ?Sized> {
    v: Option<MutexGuard<'l, T>>,
}

#[cfg(test)]
mod tests {
    use std::{
        ops::AddAssign,
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

        let a = Amp::new(5);
        a += 1;
        Amp::try_lock(&a);
        println!("Amp: {:?}", a)
    }
}
