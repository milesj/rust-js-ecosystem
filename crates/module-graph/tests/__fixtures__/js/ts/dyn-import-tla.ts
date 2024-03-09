export {};

// Side-effects
await import("./shared");

// Default
const ns = await import("./shared");

// Named
const { number, string } = await import("./shared");

// Renamed
let { number: renumber } = await import("./shared");
