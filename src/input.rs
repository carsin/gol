use std::io::{stdin, Read};
use std::sync::mpsc::{channel, Sender, Receiver};

pub fn start_input_receiver() -> Receiver<char> {
    let (input_sender, input_receiver): (Sender<char>, Receiver<char>) = channel();
    std::thread::spawn(move || {
        loop {
            let mut buf = [0u8; 1];
            stdin().read_exact(&mut buf).unwrap();
            input_sender.send(buf[0] as char).unwrap();
        }
    });

    input_receiver
}
