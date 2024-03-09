require("./shared.cjs");

function foo() {
  const ns = require("./shared.cjs");
}

function bar() {
  return () => {
    const { number } = require("./shared.cjs");
  };
}

class Baz {
  method() {
    const { string } = require("./shared.cjs");
  }
}
