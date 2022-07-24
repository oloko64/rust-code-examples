"use strict";
exports.__esModule = true;
var fs = require("fs");
try {
    var data = fs.readFileSync('../test.json', 'utf8');
    //   console.log(data);
    var teste = JSON.parse(data);
    console.log(teste);
}
catch (err) {
    console.error(err);
}
