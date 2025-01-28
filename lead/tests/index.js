const hello = require("./hello");

require("./app");

while (true) {
  new hello().init(true);
}
