# Miniscript JS


## Install
```bash
$ npm install --save miniscript-js
```

## Usage
```js
const miniscript = require('miniscript-js');

const ms = miniscript.parsePolicy('or(pk(A),pk(B))', 'STRING');
console.log(ms.semantic.minimumNKeys());
// => 2

console.log(ms.script.encode());
// => OP_PUSHBYTES_33 022222222222222222222222222222222222222222222222222222222222222222 OP_CHECKSIGVERIFY OP_PUSHBYTES_33 022222222222222222222222222222222222222222222222222222222222222222 OP_CHECKSIG
```

## License
MIT
