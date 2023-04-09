use std::{fmt, sync::Once, time};

pub struct Singleton {
    date: String,
}

impl Singleton {
    pub fn get_instance() -> &'static Singleton {
        static mut INSTANCE: Option<Singleton> = None;
        static INIT: Once = Once::new();

        INIT.call_once(|| unsafe {
            INSTANCE = Some(Singleton {
                date: time::SystemTime::now()
                    .duration_since(time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
            });
        });

        unsafe { INSTANCE.as_ref().unwrap() }
    }
}

impl fmt::Display for Singleton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.date)
    }
}

pub fn test_singleton() {
    for _ in 0..10 {
        let singleton = Singleton::get_instance();
        println!("{}", singleton);
    }
}
