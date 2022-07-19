import * as wasm from '../wasm-rust/pkg/wasm_rust.js'

function calculateFibonacci(n) {
    if (n < 2) {
        return n;
    }
    return calculateFibonacci(n - 1) + calculateFibonacci(n - 2);
}


let teste = wasm.add(1, 2)

console.time('wasm')
let result = wasm.calc_fibonacci(45)
console.timeEnd('wasm')

console.time('node.js')
let result2 = calculateFibonacci(45)
console.timeEnd('node.js')

console.log(teste)

console.log(result)
console.log(result2)
