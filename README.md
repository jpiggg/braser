![headline](docs/assets/headline.png)

# Braser
The fast serialiser written in Rust! 

Note! Right now it can be used in raw nodejs apps only, but using with bundlers (webpack/rollup) come soon!

## Motivation

With braser you could serialise a lot of different data types without wastes. It encodes data to string and then decode it to the same data types as initial were. 

Right now we support those types of data:

|  Name | Supported |
| -------- | -------- |
| String| yes|
| Number|yes |
| BigInt|yes |
| Infinity|yes |
| Date|yes |
| Boolean|yes |
| NaN |yes |
| Undefined |yes |
| Null |yes |
| Function |yes |
| Object |yes |
| Array |yes |
| Class | No |
| Symbol | No |


## Quick start

```js
const br = require("braser");

console.log(br.decode(br.encode({foo: "bar"})));

```

## Usage

The braser supports different targets, depending on your needs.

### Usage in Browser

// @TODO 
### Usage in Nodejs

```js
const br = require("braser");

console.log(br.decode(br.encode({foo: "bar"})));

```

### Usage with Typescript/Webpack/Rollup/etc.

```js
import br from "braser";

console.log(br.decode(br.encode({foo: "bar"})));

```

## Use cases

In the most cases you have to transfer data between different storages. For example, you could use serialisation with such persistent storages:


- Cache API
- Cookies
- DOM Storage (Local Storage)
- File System API (browser-provided and sandboxed file system)
- IndexedDB
- Service workers


### Comparing with JSON

----------------
**Undefined transformation**

```typescript
JSON.parse(JSON.stringify({ test: undefined })); // {}
```

```typescript
br.decode(br.encode({ test: undefined })); // { test: undefined }
```

----------------

**NaN transformation**

```typescript
JSON.parse(JSON.stringify({ foo: NaN })); // {foo: null}
```

```typescript
JSON.parse(JSON.stringify({ foo: NaN })); // { foo: NaN }
```
----------------
**Lost Date instance**

```typescript
const res = JSON.parse(JSON.stringify({ test: new Date() })); // { test: '2022-08-16T13:04:28.698Z' }
res['test'] instanceof Date; // false

new Date() instanceof Date; // true
```

```typescript
const res = br.decode(br.encode({ test: new Date() })); // { test: 2022-08-16T13:04:28.698Z }
res['test'] instanceof Date; // true

new Date() instanceof Date; // true
```
----------------
**Function support**

```typescript
JSON.parse(JSON.stringify({ f: () => 'test' })) // {}
```

```typescript
br.decode(br.encode({ f: () => 'test' })) // {f: function f()}
```

## Typed errors

// @ TODO


*Powered by*

|  |  |
| -------- | ------- |
|   <img src="docs/assets/rust.png" alt="rust" width="200" height="200"/>  | <img src="docs/assets/wa.png" alt="web_assembly" width="200" height="200"/> |