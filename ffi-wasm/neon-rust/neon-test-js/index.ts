// import { createRequire } from 'node:module';
// const require = createRequire(import.meta.url);
const { fibonacci,cpu_threads, js_functions, multi_thread } = require("../index.node");

console.time('fibonacci-rust');
console.log(fibonacci(200));
console.timeEnd('fibonacci-rust');

console.time('fibonacci-js');
console.log(fibonacciJs(200));
console.timeEnd('fibonacci-js');

console.log('cpu threads', cpu_threads());

multi_thread();

console.log('parseInt from Rust: ', js_functions());

function fibonacciJs(num: number): { result: number } {
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
