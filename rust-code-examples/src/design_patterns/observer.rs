trait Observer {
    fn id(&self) -> &str;
    fn update(&self, video: &str);
}

struct YouTube {
    subscribers: Vec<Box<dyn Observer>>,
    name: String,
}

impl YouTube {
    fn new(name: &str) -> Self {
        Self {
            subscribers: vec![],
            name: name.to_string(),
        }
    }

    fn subscribe(&mut self, subscriber: Box<dyn Observer>) {
        self.subscribers.push(subscriber);
    }

    fn unsubscribe(&mut self, subscriber: Box<dyn Observer>) {
        self.subscribers.retain(|s| s.id() != subscriber.id());
    }

    fn notify(&self, video: &str) {
        for subscriber in &self.subscribers {
            subscriber.update(video);
        }
    }
}

struct Subscriber {
    id: String,
}

impl Subscriber {
    fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl Observer for Subscriber {
    fn id(&self) -> &str {
        &self.id
    }

    fn update(&self, video: &str) {
        println!("{}: {}", self.id, video);
    }
}

pub fn observer() {
    let mut youtube = YouTube::new("Rust");
    let subscriber1 = Box::new(Subscriber::new("John"));
    let subscriber2 = Box::new(Subscriber::new("Jane"));

    youtube.subscribe(subscriber1);
    youtube.subscribe(subscriber2);

    youtube.notify("New video: Rust for beginners");
}
