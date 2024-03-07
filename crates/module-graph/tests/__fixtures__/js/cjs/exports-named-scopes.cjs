exports.root = true;

function bar() {
  // This is bad but valid?
  exports.one = 1;
}

bar();

(function () {
  {
    exports.two = 2;
  }
})();
