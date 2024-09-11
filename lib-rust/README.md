# Make WebAssembly

## 1. Install rust and wasm-pack
This environment is for WSL2(Windows Subsystem for Linux, Ubuntu 22.04).
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new --lib project-name
```

## 2. Build wasm
```bash
cd project-name
wasm-pack build --target web
```

## 3. Using wasm
just import wasm and js files from pkg folder.

```js
import init, { myFunction } from "./pkg/my_wasm.js"
```

<br/>
<br/>

# Using wasm-bindgen

## 1. Types on js and wasm-bindgen
reference type with slice on rust -> js  
&[int32] on rust -> Int32Array on js

reference : https://rustwasm.github.io/wasm-bindgen/reference/reference-types.html


## 2. Use struct on wasm-bindgen
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct myStruct {}

#[wasm_bindgen]
impl myStruct {
    #[wasm_bindgen(constructor)]
    pub fn new() -> myStruct {
        myStruct {}
    }

    #[wasm_bindgen]
    pub fn necessary_function() {}
}
```


