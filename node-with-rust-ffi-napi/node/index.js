const ffi = require('ffi-napi');

const lib = ffi.Library('../rust/target/release/librust_math', {
    'add_numbers': ['int', ['int', 'int']],
    'calc_fibonacci': ['int', ['int']],
})

function calculateFibonacci(n) {
    if (n < 2) {
        return n;
    }
    return calculateFibonacci(n - 1) + calculateFibonacci(n - 2);
}

console.log(`Adding numbers: ${lib.add_numbers(10, 10)}`);

console.log('\n')

console.log('\n#############################################');
console.log('Calculating fibonacci numbers one time');
console.log('#############################################\n');

console.log('Calculating fibonacci using Rust with exported C dynamic library');
console.time('Single run using Rust with exported C dynamic library')
console.log(`Fibonacci of 45 (Rust): ${lib.calc_fibonacci(45)}`);
console.log('')
console.timeEnd('Single run using Rust with exported C dynamic library')

console.log('\n');

console.log('Calculating fibonacci in Node');
console.time('Single run in Node')
console.log(`Fibonacci of 45 (Node): ${calculateFibonacci(45)}`);
console.log('')
console.timeEnd('Single run in Node')

console.log('\n#############################################');
console.log('#############################################\n');

console.log('\n#############################################');
console.log('Calculating fibonacci numbers five times');
console.log('#############################################\n');

console.log('Calculating fibonacci using Rust with exported C dynamic library (5 runs average)');
console.time('Total time using Rust with exported C dynamic library')
const startRust5Runs = Date.now()
for (let i = 0; i < 5; i++) {
    console.log(`Fibonacci of 45 (Rust): ${lib.calc_fibonacci(45)} round ${i + 1}`);
}
const stopRust5Runs = Date.now()
console.log('')
console.timeEnd('Total time using Rust with exported C dynamic library')
console.log(`Rounded time taken in 5 runs: ${(((stopRust5Runs - startRust5Runs) / 5) /1000).toFixed(3)}s`);

console.log('\n');

console.log('Calculating fibonacci in Node (5 runs average)');
console.time('Total time using Node')
const startNode5Runs = Date.now()
for (let i = 0; i < 5; i++) {
    console.log(`Fibonacci of 45 (Node): ${calculateFibonacci(45)} round ${i + 1}`);
}
const stopNode5Runs = Date.now()
console.log('')
console.timeEnd('Total time using Node')
console.log(`Rounded time taken in 5 runs: ${(((stopNode5Runs - startNode5Runs) / 5) / 1000).toFixed(3)}s`);

console.log('\n#############################################');
console.log('#############################################\n');
