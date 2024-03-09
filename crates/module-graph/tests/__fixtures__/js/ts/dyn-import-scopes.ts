export {};

await import("./shared");

async function foo() {
  const ns = await import("./shared");
}

async function bar() {
  return async () => {
    const { number } = await import("./shared");
  };
}

class Baz {
  async method() {
    const { string } = await import("./shared");
  }
}
