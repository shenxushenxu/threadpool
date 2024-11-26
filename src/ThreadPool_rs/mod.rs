use std::cell::RefCell;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{mpsc, Arc, LazyLock, Mutex};

mod Executor_rs;
mod Message_rs;
pub mod Future_rs;

use Executor_rs::Executor;
use Message_rs::Message;
use Future_rs::Future;


static mut CORS_THREAD: LazyLock<Mutex<RefCell<usize>>> = std::sync::LazyLock::new(|| {
    return Mutex::new(RefCell::new(0));
}
);

pub struct ThreadPool {
    sync_sender: SyncSender<Message>,

    arc_mutex_receiver: Arc<Mutex<Receiver<Message>>>,

    core_pool_size: usize,
    maximum_pool_size: usize,
    keep_alive_time: usize,
}

type Job = Box<dyn Fn() + Send + 'static>;

impl ThreadPool {
    pub fn new(core_pool_size: usize, maximum_pool_size: usize, keep_alive_time: usize) -> Self {
        let (sync_sender, receiver) = mpsc::sync_channel::<Message>(keep_alive_time);

        let arc_mutex_receiver = Arc::new(Mutex::new(receiver));


        for _ in 0..core_pool_size {
            let clone_receiver = Arc::clone(&arc_mutex_receiver);

            Executor::new(clone_receiver);
        }


        return ThreadPool {
            sync_sender: sync_sender,
            arc_mutex_receiver: arc_mutex_receiver,
            core_pool_size: core_pool_size,
            maximum_pool_size: maximum_pool_size,
            keep_alive_time: keep_alive_time,
        };
    }


    pub fn executor<F>(&self, closure: F) -> Future
    where
        F: Fn() + Send + 'static,
    {

        unsafe {

            let lock = (*CORS_THREAD).lock().unwrap();
            let mut thread_size = lock.borrow_mut();


            let (sender, receiver) = mpsc::channel::<String>();

            self.sync_sender.send(Message::Mess_job((Box::new(closure), sender))).unwrap();

            let future = Future {receiver:receiver};

            *thread_size = (*thread_size) + 1;

            if (*thread_size) >= self.core_pool_size && (*thread_size) <= self.maximum_pool_size {
                Executor::new_thread(Arc::clone(&self.arc_mutex_receiver));

                *thread_size = (*thread_size) + 1;
            }



            return future;

        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {}
}
