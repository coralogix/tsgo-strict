// Uses globals from mock-lib/globals (loaded via types)
describe("suite", () => {
  it("test", () => {
    // implicit any — should trigger strict error
    const fn = (x) => x;
    fn(1);
  });
});
