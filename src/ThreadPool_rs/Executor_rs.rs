use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::ThreadPool_rs::Message_rs::Message;
use crate::ThreadPool_rs::CORS_THREAD;


pub struct Executor {
    handle: JoinHandle<()>,
}
impl Executor {
    pub fn new(clone_receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let head = thread::spawn(move || {
            loop {
                let mess = clone_receiver.lock().unwrap().recv().unwrap();
                match mess {
                    Message::Mess_job((closure, sender)) => {
                        closure();
                        sender.send(String::from("end")).unwrap();

                        unsafe {
                            let lock = (*CORS_THREAD).lock().unwrap();
                            let mut thread_size = lock.borrow_mut();
                            (*thread_size) = ((*thread_size) - 1);
                        }
                    }
                    Message::Break => {
                        break;
                    }
                }
            }
        });

        return Executor { handle: head };
    }

    pub fn new_thread(receiver: Arc<Mutex<Receiver<Message>>>){
        match receiver.lock().unwrap().try_recv() {
            Ok(message) => {
                match message {
                    Message::Mess_job((closure, sender)) => {
                        thread::spawn(move || {
                            closure();
                            sender.send(String::from("end")).unwrap();
                            unsafe {
                                let lock = (*CORS_THREAD).lock().unwrap();
                                let mut thread_size = lock.borrow_mut();
                                (*thread_size) = ((*thread_size) - 1);
                            }
                        });
                    }
                    Message::Break => {}
                }
            }
            Err(e) => ()
        }
    }
}