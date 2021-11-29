mod tests;

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

// TODO
pub struct AmpContainer<'l, T: ?Sized> {
    v: Option<MutexGuard<'l, T>>,
}

/*
    Allows access to the inner methods from T.
*/
mod deref_impl {
    use std::ops::{Deref, DerefMut};

    use crate::Amp;

    impl<T> Deref for Amp<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe {
                let mg = self.v.lock().unwrap();
                let k = mg.deref() as *const T;
                &*k as &T
            }
        }
    }

    impl<T> DerefMut for Amp<T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe {
                let mut mg = self.v.lock().unwrap();
                let k = mg.deref_mut() as *mut T;
                &mut *k as &mut T
            }
        }
    }
}
