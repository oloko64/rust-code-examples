// factory pattern

class Burger {
    private name: string;
    private price: number;
    private calories: number;

    constructor() {
        this.name = "";
        this.price = 0;
        this.calories = 0;
    }

    private createBurger(name: string, price: number, calories: number) {
        const burger = new Burger();
        burger.name = name;
        burger.price = price;
        burger.calories = calories;
        return burger;
    }

    createCheeseBurger() {
        return this.createBurger("Cheese Burger", 10, 200);
    }

    createChickenBurger() {
        return this.createBurger("Chicken Burger", 15, 250);
    }

    createFishBurger() {
        return this.createBurger("Fish Burger", 12, 220);
    }
}

const burgerFactory = new Burger();
const cheeseBurger = burgerFactory.createCheeseBurger();
const chickenBurger = burgerFactory.createChickenBurger();
const fishBurger = burgerFactory.createFishBurger();