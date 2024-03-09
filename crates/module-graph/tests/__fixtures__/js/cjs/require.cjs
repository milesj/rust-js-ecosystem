// Side-effect
require("./shared.cjs");

// Default
const def = require("./shared.cjs");

// Named
const { number, string } = require("./shared.cjs");

// Renamed
let { number: renumber } = require("./shared.cjs");

// Default named
let {
  default: { number, string },
} = require("./shared.cjs");
