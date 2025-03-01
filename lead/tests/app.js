console.time("\nTime");

const hello = new (require("./hello.mod"))();

var a = "12";

console.log(a);

hello.init(a);

console.log(a);

console.log(process.platform);

console.timeEnd("\nTime");
