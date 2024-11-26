use std::sync::mpsc::Sender;



type Job = Box<dyn Fn() + Send + 'static>;
pub enum Message {
    Mess_job((Job, Sender<String>)),
    Break,
}