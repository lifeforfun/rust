use std::sync::mpsc::*;
use std::thread;

#[derive(Debug, Clone)]
struct Test<'a> {
    i: isize,
    s: &'a str,
}

pub fn test()
{
    let (sender, receiver) = channel();
    let sender_cloned = sender.clone();
    thread::spawn(move ||{
        sender.send(1).unwrap();
    });
    thread::spawn(move || {
        sender_cloned.send(2).unwrap();
    });

    let mut i = 1;
    loop {
        i+=1;
        if i>100 {
            break;
        }
        println!("{:?}", receiver.recv());
    }

}