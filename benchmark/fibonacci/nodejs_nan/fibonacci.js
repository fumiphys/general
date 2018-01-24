var addon = require('bindings')('fibonacci');

var startTime = new Date();
result = addon.fibonacci(40);
var endTime = new Date();

time = endTime - startTime;

console.log("nodejs+C++ " + result + " " + time/1000.0 + "sec.")
