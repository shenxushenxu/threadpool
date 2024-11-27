mod ThreadPool_rs;
pub use ThreadPool_rs::ThreadPool;
pub use ThreadPool_rs::Future_rs::Future;


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::sync::{LazyLock, Mutex};
    use super::*;
    static mut CORS_THREAD: LazyLock<Mutex<RefCell<usize>>> = std::sync::LazyLock::new(|| {
        return Mutex::new(RefCell::new(0));
    }
    );
    #[test]
    fn tests() {


        unsafe {

            let lock = (*CORS_THREAD).lock().unwrap();
            let mut thread_size = lock.borrow_mut();
            (*thread_size) = ((*thread_size) + 8);

            println!("{}", (*thread_size))

        }

        unsafe {

            let lock = (*CORS_THREAD).lock().unwrap();
            let mut thread_size = lock.borrow_mut();

            println!("{}", (*thread_size))

        }

    }
}