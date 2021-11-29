mod tests;

use std::{
    borrow::BorrowMut,
    ops::DerefMut,
    sync::{Arc, Mutex, MutexGuard, TryLockError},
};

#[derive(Debug, Clone)]
pub struct Amp<T> {
    v: Arc<Mutex<T>>,
}
impl<T> Amp<T> {
    pub fn new(t: T) -> Self {
        Self {
            v: Arc::new(Mutex::new(t)),
        }
    }
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
                &*k
            }
        }
    }

    impl<T> DerefMut for Amp<T> {
        fn deref_mut(&mut self) -> &mut T {
            unsafe {
                let mut mg = self.v.lock().unwrap();
                let k = mg.deref_mut() as *mut T;
                &mut *k
            }
        }
    }
}

// TODO
pub struct AmpContainer<'l, T: ?Sized> {
    v: Option<&'l mut T>,
    e: Option<TryLockError<MutexGuard<'l, T>>>,
}
impl<'l, T> AmpContainer<'l, T> {
    pub fn success(mut self, f: fn(&mut T)) -> Self {
        if let Some(mg) = &mut self.v {
            f(mg)
        }
        self
    }

    pub fn error(self, f: fn(&TryLockError<MutexGuard<'l, T>>)) -> Self {
        if let Some(e) = &self.e {
            f(e)
        }
        self
    }
}
impl<T> Amp<T> {
    pub fn try_lock(s: &Self) -> AmpContainer<T> {
        match s.v.try_lock() {
            Ok(mut v) => AmpContainer {
                v: {
                    unsafe {
                        let k = v.deref_mut() as *mut T;
                        Some(&mut *k)
                    }
                },
                e: None,
            },
            Err(e) => AmpContainer {
                v: None,
                e: Some(e),
            },
        }
    }
}
