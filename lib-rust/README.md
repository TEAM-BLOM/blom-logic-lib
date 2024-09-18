# How to use check logic lib

<b>This lib is made for the web.  
So if you want to use this lib in node.js, you cannot use js file directly.  
</b>

<b>If you want to use this lib in node.js, you should build with `wasm-pack build --target nodejs`</b>


### Using example in web [index.html](../testWeb/index.html) and [test.html](../testWeb/test.html)

## 1. Import wasm and Init module
```js
import init from './pkg/check_logic.js';

const module = await init();
```

## 2. Use functions
This lib provides functions below.  
<b>All of functions can return error(JsError), so you should handle it.</b>

- init objects:
  - `init_board(board: Int32Array) => Board` : initialize board, <b>This function already have check_valid_board logic</b>
  - `init_check(board: Board, x: number, y: number) => Check` : initialize check
- update objects:
  - `update_check_board(check: Check, new_board: Int32Array, x: number, y: number) => Check` : update check board with new board and x, y, <b>This function also have check_valid_board logic</b>
- check for a game:
  - `check_win(check: Check) => boolean` : check if the game is win
  - `check_33(check: Check) => boolean` : check if there is 33 pattern
  - `check_44(check: Check) => boolean` : check if there is 44 pattern
- check for debug:
  - `check_valid_board(board: Int32Array) => boolean` : check if the board is valid
  - `print_address(check: Check) => number` : print address of check

```js
import { init_board, 
    init_check, 
    update_check_board, 
    check_win, 
    check_33, 
    check_44, 
    check_valid_board, 
    print_address 
} from './pkg/check_logic.js';
```

<b>All of objects are reference type, so you should free it after using.</b>
```js
// example check: Check, board: Board
check.free();
board.free();
```

<br />

---

### Make WebAssembly

#### 1. Install rust and wasm-pack
This environment is for WSL2(Windows Subsystem for Linux, Ubuntu 22.04).
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new --lib project-name
```

#### 2. Build wasm
```bash
cd project-name
wasm-pack build --target web
```

#### 3. Using wasm
just import wasm and js files from pkg folder.

```js
import init, { myFunction } from "./pkg/my_wasm.js"
```

<br/>
<br/>

### Using wasm-bindgen

#### 1. Types on js and wasm-bindgen
reference type with slice on rust -> js  
&[int32] on rust -> Int32Array on js

reference : https://rustwasm.github.io/wasm-bindgen/reference/reference-types.html


#### 2. Use struct on wasm-bindgen
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


