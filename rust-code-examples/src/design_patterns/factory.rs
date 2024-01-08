#[derive(Debug)]
struct Burger {
    pub patties: i32,
    pub bacon: bool,
    pub cheese: bool,
    pub sauce: String,
    pub toasted: bool,
}

struct BurgerFactory;

impl BurgerFactory {
    fn create_burger(
        &self,
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

    fn create_cheeseburger(&self, patties: i32, bacon: bool, toasted: bool) -> Burger {
        self.create_burger(patties, bacon, true, "tomato", toasted)
    }

    fn create_baconburger(&self, patties: i32, toasted: bool) -> Burger {
        self.create_burger(patties, true, false, "tomato", toasted)
    }

    fn create_veggieburger(&self, patties: i32, toasted: bool) -> Burger {
        self.create_burger(patties, false, false, "tomato", toasted)
    }
}

fn factory() {
    let burger_factory = BurgerFactory;

    let cheeseburger = burger_factory.create_cheeseburger(2, true, true);
    let baconburger = burger_factory.create_baconburger(3, true);
    let veggieburger = burger_factory.create_veggieburger(1, false);

    println!("cheeseburger: {:?}", cheeseburger);
    println!("baconburger: {:?}", baconburger);
    println!("veggieburger: {:?}", veggieburger);
}
