const { base64_encode, base64_decode, ceaser_7} = require('../pkg/quadratic_lib.js');

console.log(base64_encode("helloworld"))
console.log(base64_decode(base64_encode("helloworld")))
console.log(ceaser_7("helloworld"))
