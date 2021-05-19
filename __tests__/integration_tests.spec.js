const {sizeof} = require("..");
const chance = require("chance")();

describe("native-sizeof", () => {
  describe("primitives", () => {
      test("returns 0 when called without an argument", () => {
        expect(sizeof()).toStrictEqual(0);
      });

    // TODO:
    // - NaN
    // - Inf
    // - Symbols
    // - Functions
      test.each`
      value | expected | description
      ${undefined} | ${0} | ${"undefined"}
      ${null} | ${0} | ${"null"}
      ${{}} | ${0} | ${"an empty object"}
      ${chance.bool()} | ${4} | ${"a boolean"}
      ${chance.integer()} | ${8} | ${"an integer number"}
      ${chance.floating()} | ${8} | ${"an float number"}
      ${chance.string({ length: 5 })} | ${5 * 2} | ${"a string of size 5"}
      ${Buffer.alloc(5)} | ${5} | ${"a buffer of size 5"}
      `("returns $expected for $description", ({ value, expected }) => {
        expect(sizeof(value)).toStrictEqual(expected);
      })
  })

  describe("arrays", () => {
    test.each`
    array | expected | description
    ${[chance.integer(), chance.floating(), chance.integer()]} | ${8 * 3} | ${"an array with elements of the same type (3 numbers)"}
    ${[chance.integer(), chance.string({ length: 3 })]} | ${8  + 3 * 2} | ${"an array with elements of the different types (1 number and 1 string of length 3)"}
    ${[null, undefined, {}, chance.integer()]} | ${8} | ${"an array with some non counted elements (null, undefined, empty object and a number)"}
    `("returns $expected for $description", ({array, expected}) => {
      expect(sizeof(array)).toStrictEqual(expected);
    })
  })

  // TODO: test cyclic object
  describe("objects", () => {
    test.each`
    object | expected | description
    ${{[chance.string({ length: 3 })]: chance.string({ length: 5 })}} | ${3 * 2 + 5 * 2} | ${"a simple object (string key of size 3 and a string value of size 5)"}
    ${{[chance.string({ length: 5})]: [chance.string({ length: 3 })]}} | ${5 * 2 + 3 * 2} | ${"an object with an array value (string key of size 5 and array value with a string of size 3)"}
    ${{[chance.integer({ min: 100, max: 999 })]: {[chance.string({ length: 3 })]: chance.string({length: 5})}}} | ${3 * 2 + 3 * 2 + 5 * 2} | ${"an object with nested objects (number key which is casted into a string, nested string key of size 3 and a nested value of size 5)"}
    `("returns $expected for $description", ({object, expected}) => {
      expect(sizeof(object)).toStrictEqual(expected);
    })
  })

  describe("large items", () => { })

})
