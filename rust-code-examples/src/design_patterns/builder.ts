// builder pattern

interface CarBuilder {
    name(name: string): CarBuilder;
    color(color: string): CarBuilder;
    maxSpeed(maxSpeed: number): CarBuilder;
    build(): Car;
}

class Car {
    name: string;
    color: string;
    maxSpeed: number;
}

class CarBuilder implements CarBuilder {
    private car: Car;

    constructor() {
        this.car = new Car();
    }

    name(name: string) {
        this.car.name = name;
        return this;
    }

    color(color: string) {
        this.car.color = color;
        return this;
    }

    maxSpeed(maxSpeed: number) {
        this.car.maxSpeed = maxSpeed;
        return this;
    }

    build() {
        return this.car;
    }

}

const carBuilder = new CarBuilder();
const car = carBuilder.name("BMW").color("red").maxSpeed(250).build();

console.log(car);