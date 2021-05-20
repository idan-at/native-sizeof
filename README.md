# native-sizeof

A native package to quickly calculate an object size, written in Rust.

## Installation
`npm install --save native-sizeof`

## Usage
```js
const { sizeof } = require('native-sizeof');

const object = {
  'key': 'value'
}

sizeof(object); // returns the size of the object (8 character => 2 * 8 = 16)
sizeof("hello"); // 5 * 2 = 10
sizeof(123) // 8 for a number
sizeof(true) // 2 for a boolean

sizeof(null) // 0 for null, undefined
```

## Summary

In javascript, there is no manual memory management, and therefore no `sizeof` capabilities, unlike C and other system languages.

However, sometimes it is useful to know how much size your object takes, so it will provide an approximation of how much memory is confused by your app, and how much memory is actually needed by your VM / Container.

Since iterating over an object and summing up the occupied memory is a synchronous task, which blocks the main thread, this library is written in Rust for  better performance.

## Object size calculation
According to the [ECMAScript® Language Specification](https://tc39.es/ecma262/):
- Each `boolean` value is represented using **2 bytes**.
- Each `number` value is represented using **8 bytes**.
- Each `string` value is represented using **2 bytes per character**. (so a string of length 3 takes 2 * 3 = 6 bytes).

NOTE: Some JS engines might optimize those numbers, but those are a good upper bound.

## Development

### Prerequisites
- Node
- Rust

### Building
- `npm install`

This will install the relevant development packages from npm and will build the binary from the rust code.

### Testing
- `npm test`

This runs both Rust tests and JS integration tests.
