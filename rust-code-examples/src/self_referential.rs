use std::{cell::RefCell, rc::Rc};

struct TextField {
    name: String,
    value: String,
}

impl TextField {
    fn new(name: &str) -> TextField {
        TextField {
            name: name.to_string(),
            value: String::new(),
        }
    }

    fn enter_character(&mut self, c: char) {
        self.value.push(c);
    }

    fn delete_character(&mut self) {
        self.value.pop();
    }
}

enum Editing {
    Name,
    Email,
}

struct App {
    name: Rc<RefCell<TextField>>,
    age: u8,
    email: Rc<RefCell<TextField>>,
    currently_editing: Option<Rc<RefCell<TextField>>>,
}

impl App {
    fn new() -> App {
        App {
            name: Rc::new(RefCell::new(TextField::new("name"))),
            age: 0,
            email: Rc::new(RefCell::new(TextField::new("email"))),
            currently_editing: None,
        }
    }

    fn toggle_editing(&mut self, editing: Editing) {
        self.currently_editing = match self.currently_editing {
            Some(_) => None,
            None => match editing {
                Editing::Name => Some(Rc::clone(&self.name)),
                Editing::Email => Some(Rc::clone(&self.email)),
            },
        };
    }

    fn enter_character(&mut self, c: char) {
        if let Some(ref currently_editing) = self.currently_editing {
            currently_editing.borrow_mut().enter_character(c);
        }
    }

    fn delete_character(&mut self) {
        if let Some(ref currently_editing) = self.currently_editing {
            currently_editing.borrow_mut().delete_character();
        }
    }
}

pub fn run_example() {
    let mut app = App::new();
    app.toggle_editing(Editing::Name);
    app.enter_character('H');
    app.enter_character('e');
    app.enter_character('l');
    app.enter_character('l');
    app.enter_character('o');
    app.delete_character();
    app.delete_character();
    app.delete_character();

    println!("Name: {}", app.name.borrow().value);
    println!("Email: {}", app.email.borrow().value);
}
