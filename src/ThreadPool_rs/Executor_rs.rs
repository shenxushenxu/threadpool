use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::ThreadPool_rs::Message_rs::Message;
use crate::ThreadPool_rs::CORS_THREAD;
use crate::ThreadPool_rs::NON_CORE_THREAD;


pub struct Executor {
    handle: JoinHandle<()>,
}
impl Executor {
    pub fn new(clone_receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let head = thread::spawn(move || {
            loop {
                let receiver = clone_receiver.lock();
                match receiver {
                    Ok(rece)=>{
                        let mess = rece.recv();
                        if let Ok(me) = mess  {
                            match me {
                                Message::Mess_job((closure, sender)) => {

                                    drop(rece);


                                    closure();
                                    sender.send(String::from("end")).unwrap();

                                    unsafe {
                                        let mut thread_size = CORS_THREAD.lock().unwrap();
                                        (*thread_size) = ((*thread_size) - 1);
                                    }
                                }
                                Message::Break => {
                                    break;
                                }
                            }
                        }

                    },
                    Err(e) => ()
                }

            }
        });

        return Executor { handle: head };
    }

    pub fn new_thread(receiver: Arc<Mutex<Receiver<Message>>>){
        let mute = receiver.try_lock();
            match mute{
                Ok(rece) => {
                    match rece.try_recv() {
                        Ok(message) => {

                            drop(rece);

                            match message {
                                Message::Mess_job((closure, sender)) => {
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
                                },

                                Message::Break => {}
                            }
                        },

                        Err(e) => ()
                    }
                },
                Err(e) => ()
            }


    }
}