// strategy pattern

trait Strategy {
    fn execute(&self, a: i32, b: i32) -> i32;
}

struct Add;

impl Strategy for Add {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct Subtract;

impl Strategy for Subtract {
    fn execute(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Self { strategy }
    }

    fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = strategy;
    }

    fn execute(&self, a: i32, b: i32) -> i32 {
        self.strategy.execute(a, b)
    }
}

pub fn strategy() {
    let context = Context::new(Box::new(Add));
    println!("10 + 5 = {}", context.execute(10, 5));

    let mut context = Context::new(Box::new(Subtract));
    println!("10 - 5 = {}", context.execute(10, 5));

    context.set_strategy(Box::new(Add));
    println!("10 + 5 = {}", context.execute(10, 5));
}
