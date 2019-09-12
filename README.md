# Miniscript JS


## Install
```bash
$ npm install --save miniscript-js
```

## Usage
```js
const miniscript = require('miniscript-js');

const ms = miniscript.parsePolicy('or(pk(A),pk(B))', 'STRING');

```
