const lib = require('../index.node');

console.time('fibonacci-rust');
console.log(lib.fibonacci(200));
console.timeEnd('fibonacci-rust');

console.time('fibonacci-js');
console.log(fibonacci(200));
console.timeEnd('fibonacci-js');

console.log('cpu threads', lib.cpu_threads());

function fibonacci(num: number): { result: number } {
    if (num < 2) {
        return {result: num};
    }
    const result = fibonacciMemo(num, []);
    return { result }
}

function fibonacciMemo(num: number, arr: number[]): number {
    if (arr[num] !== undefined) {
        return arr[num];
    }
    if (num < 2) {
        return num;
    }
    arr[num] = fibonacciMemo(num - 1, arr) + fibonacciMemo(num - 2, arr);
    return arr[num];
}
