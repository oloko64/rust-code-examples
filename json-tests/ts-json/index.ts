import * as fs from 'fs';

export interface Teste {
    products: Product[];
}

export interface Product {
    name:        string;
    price:       string;
    description: string;
}

try {
  const data = fs.readFileSync('../test.json', 'utf8');
//   console.log(data);
    const teste: Teste = JSON.parse(data);
    console.log(teste);
} catch (err) {
  console.error(err);
}
