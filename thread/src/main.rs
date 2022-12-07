use std::{thread::{spawn, sleep}, time::Duration, sync::mpsc};

fn main() {
    let (sender, receiver) = mpsc::channel();
    let sender1 = sender.clone();

    spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ];

        for val in vals {
            println!("send msg is {}", val);
            sender.send(val).unwrap();
            sleep(Duration::from_secs(1));
        }
    });

    spawn(move || {
        let vals = vec![
            String::from("f"),
            String::from("g"),
            String::from("h"),
            String::from("i"),
            String::from("j"),
        ];

        for val in vals {
            println!("sender1 msg is {}", val);
            sender1.send(val).unwrap();
            sleep(Duration::from_secs(1));
        }
    });
    
    for msg in receiver {
        // let msg = r.recv().unwrap();
        println!("revive msg:{:?}", msg);
    }
    
}
