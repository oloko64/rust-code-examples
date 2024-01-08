// strategy pattern

interface Strategy {
    execute(a: number, b: number): number;
}

class Add implements Strategy {
    execute(a: number, b: number) {
        return a + b;
    }
}

class Subtract implements Strategy {
    execute(a: number, b: number) {
        return a - b;
    }
}

class Context {
    private strategy: Strategy;

    constructor(strategy: Strategy) {
        this.strategy = strategy;
    }

    setStrategy(strategy: Strategy) {
        this.strategy = strategy;
    }

    executeStrategy(a: number, b: number) {
        return this.strategy.execute(a, b);
    }
}

const context = new Context(new Add());
console.log(context.executeStrategy(1, 2));

context.setStrategy(new Subtract());
console.log(context.executeStrategy(1, 2));