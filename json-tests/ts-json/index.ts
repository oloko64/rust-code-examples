import * as fs from 'fs';
import { z } from "zod";

export interface Teste {
    products: Product[];
}

export interface Product {
    name:        string;
    price:       string;
    description: string;
}

const myTestSchema = z.object({
    products: z.array(z.object({
        name:        z.string(),
        price:       z.string(),
        description: z.string(),
    }))
});

try {
  const data = fs.readFileSync('../test.json', 'utf8');
//   console.log(data);
    const teste = myTestSchema.parse(JSON.parse(data));
    console.log(teste);
} catch (err) {
  console.error(err);
}
