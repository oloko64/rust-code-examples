"use strict";
exports.__esModule = true;
var fs = require("fs");
var zod_1 = require("zod");
var myTestSchema = zod_1.z.object({
    products: zod_1.z.array(zod_1.z.object({
        name: zod_1.z.string(),
        price: zod_1.z.string(),
        description: zod_1.z.string()
    }))
});
try {
    var data = fs.readFileSync('../test.json', 'utf8');
    //   console.log(data);
    var teste = myTestSchema.parse(JSON.parse(data));
    console.log(teste);
}
catch (err) {
    console.error(err);
}
