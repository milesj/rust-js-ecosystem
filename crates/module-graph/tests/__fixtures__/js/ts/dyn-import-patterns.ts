export {};

// Default
const { default: def } = await import("./shared-def-object");

// Destructure
const { number } = await import("./shared");

// Destructure with rest
const { string, ...rest } = await import("./shared");

// Destructure nested object
const {
  default: {
    object: {
      one: {
        two: { value },
      },
    },
  },
} = await import("./shared-def-object");

// Assignment, unknown export
const { unknown = "abc" } = await import("./shared");
