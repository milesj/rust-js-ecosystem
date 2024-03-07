async function test() {
  // Default
  const { default: def } = await import("./shared-def-object.cjs");

  // Destructure
  const { number } = await import("./shared.cjs");

  // Destructure with rest
  const { string, ...rest } = await import("./shared.cjs");

  // Destructure nested object
  const {
    default: {
      object: {
        one: {
          two: { value },
        },
      },
    },
  } = await import("./shared-def-object.cjs");
}
