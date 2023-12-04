use std::{sync::mpsc::Sender, thread};

pub fn example() {
    let (sender, receiver) = std::sync::mpsc::channel::<Kind>();

    spawn_thread_task(sender.clone(), 1);
    spawn_thread_task(sender.clone(), 2);
    spawn_thread_task(sender, 3);

    while let Ok(message) = receiver.recv() {
        match message {
            Kind::Connect => println!("Connected"),
            Kind::Disconnect => println!("Disconnected"),
            Kind::Message(message) => println!("Message: {:?}", message),
        }
    }
}

fn spawn_thread_task(sender: Sender<Kind>, sleep_time: u64) {
    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 0..10 {
            thread::sleep(std::time::Duration::from_secs(sleep_time));

            if i == 0 {
                sender.send(Kind::Connect).unwrap();
                continue;
            } else if i == 9 {
                sender.send(Kind::Disconnect).unwrap();
                continue;
            }

            sender
                .send(Kind::Message(Message {
                    text: format!("Message #{} from thread {:?}", i, thread_id),
                    date: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }))
                .unwrap();
        }
    });
}

#[derive(Debug)]
enum Kind {
    Connect,
    Disconnect,
    Message(Message),
}

#[derive(Debug)]
struct Message {
    text: String,
    date: u64,
}
