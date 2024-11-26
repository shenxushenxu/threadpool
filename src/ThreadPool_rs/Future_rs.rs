use std::sync::mpsc::Receiver;

pub struct  Future {
    pub receiver: Receiver<String>
}

impl Future {
    pub fn get(self) {

        match self.receiver.recv(){
            Ok(end) => {
                match end.as_str() {
                    "end" => (),
                    _ => println!("报错了")
                }
            },
            Err(e) => println!("{:#?}", e)
        }
    }
}
