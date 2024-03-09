// Default
const { default: def } = await import("./shared-def-object.mjs");

// Destructure
const { number } = await import("./shared.mjs");

// Destructure with rest
const { string, ...rest } = await import("./shared.mjs");

// Destructure nested object
const {
  default: {
    object: {
      one: {
        two: { value },
      },
    },
  },
} = await import("./shared-def-object.mjs");

// Assignment, unknown export
const { unknown = "abc" } = await import("./shared.mjs");
