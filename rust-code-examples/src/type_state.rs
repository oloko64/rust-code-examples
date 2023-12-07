struct Locked;
struct Unlocked;

struct Door<State = Locked> {
    color: String,
    state: std::marker::PhantomData<State>,
}

// This will prevent the user from creating a Door and passing the wrong state
impl Door {
    fn new(color: impl Into<String>) -> Self {
        Door {
            color: color.into(),
            state: Default::default(),
        }
    }
}

// If you want to allow the user to create a Door with a specific state, you can create the `new` method inside this `impl` block
impl<State> Door<State> {
    // fn new(color: impl Into<String>) -> Self {
    //     Door {
    //         color: color.into(),
    //         state: Default::default(),
    //     }
    // }

    fn color(&self) -> &str {
        &self.color
    }
}

impl Door<Locked> {
    fn open(self) -> Door<Unlocked> {
        println!("Opening the {color} door", color = self.color);

        Door {
            color: self.color,
            state: std::marker::PhantomData::<Unlocked>,
        }
    }
}

impl Door<Unlocked> {
    fn close(self) -> Door<Locked> {
        println!("Closing the {color} door", color = self.color);

        Door {
            color: self.color,
            state: std::marker::PhantomData::<Locked>,
        }
    }
}

pub fn example() {
    let door = Door::new("Red");
    println!("The door's color is {}", door.color());

    let door = door.open();
    let door = door.close();
}
