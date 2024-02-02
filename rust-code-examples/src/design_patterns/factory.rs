#[non_exhaustive]
#[derive(Debug)]
struct Burger {
    patties: i32,
    bacon: bool,
    cheese: bool,
    sauce: String,
    toasted: bool,
}

impl Burger {
    fn create_burger(
        patties: i32,
        bacon: bool,
        cheese: bool,
        sauce: &str,
        toasted: bool,
    ) -> Burger {
        Burger {
            patties,
            bacon,
            cheese,
            sauce: sauce.to_string(),
            toasted,
        }
    }

    fn create_cheeseburger(patties: i32, bacon: bool, toasted: bool) -> Burger {
        Burger::create_burger(patties, bacon, true, "tomato", toasted)
    }

    fn create_baconburger(patties: i32, toasted: bool) -> Burger {
        Burger::create_burger(patties, true, false, "tomato", toasted)
    }

    fn create_veggieburger(patties: i32, toasted: bool) -> Burger {
        Burger::create_burger(patties, false, false, "tomato", toasted)
    }
}

fn factory() {
    let cheeseburger = Burger::create_cheeseburger(2, true, true);
    let baconburger = Burger::create_baconburger(3, true);
    let veggieburger = Burger::create_veggieburger(1, false);

    println!("cheeseburger: {:?}", cheeseburger);
    println!("baconburger: {:?}", baconburger);
    println!("veggieburger: {:?}", veggieburger);
}
