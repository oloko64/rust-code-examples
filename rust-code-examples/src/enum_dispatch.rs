use enum_dispatch::enum_dispatch;

struct MyImplementorA;

impl MyImplementorA {
    fn new() -> Self {
        Self
    }
}

impl MyBehavior for MyImplementorA {
    fn my_trait_method(&self) {
        println!("MyImplementorA");
    }
}

struct MyImplementorB;

impl MyImplementorB {
    fn new() -> Self {
        Self
    }
}

impl MyBehavior for MyImplementorB {
    fn my_trait_method(&self) {
        println!("MyImplementorB");
    }
}

#[enum_dispatch]
enum MyBehaviorEnum {
    MyImplementorA,
    MyImplementorB,
}

#[enum_dispatch(MyBehaviorEnum)]
trait MyBehavior {
    fn my_trait_method(&self);
}

pub fn example() {
    // Enum dispatch

    let mut enum_dispatch_var: MyBehaviorEnum = MyImplementorA::new().into();
    enum_dispatch_var = MyImplementorB::new().into();

    enum_dispatch_var.my_trait_method(); //dynamic dispatch
    call_enum(enum_dispatch_var); //static dispatch

    // Dynamic dispatch

    let mut dynamic_dispatch_var: Box<dyn MyBehavior> = Box::new(MyImplementorA::new());
    dynamic_dispatch_var = Box::new(MyImplementorB::new());

    dynamic_dispatch_var.my_trait_method(); //dynamic dispatch
    call_dynamic(&*dynamic_dispatch_var); //static dispatch
}

fn call_dynamic(var: &dyn MyBehavior) {
    var.my_trait_method();
}

fn call_enum(var: MyBehaviorEnum) {
    var.my_trait_method();
}
