use std::cell::RefCell;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::{mpsc, Arc, LazyLock, Mutex};
use std::thread;

mod Executor_rs;
mod Message_rs;
pub mod Future_rs;

use Executor_rs::Executor;
use Message_rs::Message;
use Future_rs::Future;


static mut CORS_THREAD: Mutex<usize> = Mutex::new(0);
static mut NON_CORE_THREAD: Mutex<usize> = Mutex::new(0);
pub struct ThreadPool {
    sync_sender: SyncSender<Message>,
    core_pool_size: usize,
    maximum_pool_size: usize,
    maximum_queue: usize,
}

type Job = Box<dyn Fn() + Send + 'static>;

impl ThreadPool {
    pub fn new(core_pool_size: usize, maximum_pool_size: usize, maximum_queue: usize) -> Self {

        assert!(maximum_pool_size > core_pool_size);
        assert!(maximum_queue > (core_pool_size + maximum_pool_size));


        let (sync_sender, receiver) = mpsc::sync_channel::<Message>(maximum_queue);

        let arc_mutex_receiver = Arc::new(Mutex::new(receiver));


        for _ in 0..core_pool_size {
            let clone_receiver = Arc::clone(&arc_mutex_receiver);

            Executor::new(clone_receiver);
        }


        thread::spawn(move || {
            loop {
                unsafe {
                    let mut thread_size = CORS_THREAD.lock().unwrap();
                    let mut non_thread_size = NON_CORE_THREAD.lock().unwrap();

                    if (*thread_size) > core_pool_size && (*non_thread_size) < (maximum_pool_size - core_pool_size) {
                        drop(thread_size);

                        let non_receiver = Arc::clone(&arc_mutex_receiver);

                        // Executor::new_thread(Arc::clone(&arc_mutex_receiver));
                        let mute = non_receiver.try_lock();
                        match mute {
                            Ok(rece) => {
                                match rece.try_recv() {
                                    Ok(message) => {
                                        drop(rece);

                                        match message {
                                            Message::Mess_job((closure, sender)) => {
                                                println!("非核心线程.........");
                                                thread::spawn(move || {
                                                    closure();
                                                    sender.send(String::from("end")).unwrap();
                                                    unsafe {
                                                        let mut thread_size = CORS_THREAD.lock().unwrap();
                                                        (*thread_size) = ((*thread_size) - 1);

                                                        let mut non_thread_size = NON_CORE_THREAD.lock().unwrap();
                                                        (*non_thread_size) = (*non_thread_size) - 1;
                                                    }
                                                });
                                            }

                                            Message::Break => { break }
                                        }
                                    }

                                    Err(e) => ()
                                }
                            }
                            Err(e) => ()
                        }
                        (*non_thread_size) += 1;
                    }
                }
            }
        });

        return ThreadPool {
            sync_sender: sync_sender,
            core_pool_size: core_pool_size,
            maximum_pool_size: maximum_pool_size,
            maximum_queue: maximum_queue,
        };
    }


    pub fn executor<F>(&self, closure: F) -> Future
    where
        F: Fn() + Send + 'static,
    {
        unsafe {
            let mut thread_size = CORS_THREAD.lock().unwrap();


            let (sender, receiver) = mpsc::channel::<String>();

            self.sync_sender.send(Message::Mess_job((Box::new(closure), sender))).unwrap();
            let future = Future { receiver: receiver };


            *thread_size = (*thread_size) + 1;
            return future;
        }
    }


    pub fn shutdown(&self) {
        for _ in 0..(self.core_pool_size + 1) {
            self.sync_sender.send(Message::Break).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in 0..(self.core_pool_size + 1) {
            self.sync_sender.send(Message::Break).unwrap();
        }
    }
}
