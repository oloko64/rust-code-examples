// Observer pattern

interface Subject {
    subscribe(observer: Observer): void;
    unsubscribe(observer: Observer): void;
    notifyObservers(data: any): void;
}

interface Observer {
    update(data: any): void;

}

class Youtube implements Subject {
    observers: Observer[];

    constructor() {
        this.observers = [];
    }

    subscribe(observer: Observer) {
        this.observers.push(observer);
    }

    unsubscribe(observer: Observer) {
        this.observers = this.observers.filter((obs) => obs !== observer);
    }

    notifyObservers(data: any) {
        this.observers.forEach((observer) => observer.update(data));
    }
}

class Observer implements Observer {
    name: string;

    constructor(name: string) {
        this.name = name;
    }

    update(data: any) {
        console.log(`${this.name} got notified with data: ${data}`);
    }
}

const youtube = new Youtube();

const observer1 = new Observer("Observer 1");
const observer2 = new Observer("Observer 2");

youtube.subscribe(observer1);
youtube.subscribe(observer2);

youtube.notifyObservers("First notification");