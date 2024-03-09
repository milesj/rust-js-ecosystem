await import("./shared.mjs");

async function foo() {
  const ns = await import("./shared.mjs");
}

async function bar() {
  return async () => {
    const { number } = await import("./shared.mjs");
  };
}

class Baz {
  async method() {
    const { string } = await import("./shared.mjs");
  }
}
