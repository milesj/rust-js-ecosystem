// Side-effects
await import("./shared.mjs");

// Default
const ns = await import("./shared.mjs");

// Named
const { number, string } = await import("./shared.mjs");

// Renamed
let { number: renumber } = await import("./shared.mjs");
