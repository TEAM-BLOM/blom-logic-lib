# How to use check logic lib

install: `npm i @r_seung_h/blom-logic`  
or download from below asset.
[download](https://github.com/TEAM-BLOM/blom-logic-lib/releases/download/v0.1.1/logic-web-0.1.1.zip)  
version: 0.1.1 (npm package is 0.1.3)  

<b>This lib is made for the web and bundler.</b>

# Support Functions
- <b>init objects:</b>
  - `init_obj(board: Int32Array, x: number, y: number) => Check` : initialize check with board and x, y
- <b>update objects:</b>
  - `update_check_board(check: Check, new_board: Int32Array, x: number, y: number) => Check` : update check board with new board and x, y, <b>This function also have check_valid_board logic</b>
- <b>check for a game:</b>
  - `check_win(check: Check) => boolean` : check if the game is win
  - `check_33(check: Check) => boolean` : check if there is 33 pattern
  - `check_44(check: Check) => boolean` : check if there is 44 pattern
- <b>check for debug:</b>
  - `check_valid_board(board: Int32Array) => boolean` : check if the board is valid
  - `print_address(check: Check) => number` : print address of check
- <b>free objects:</b>  
  - `free(check: Check) => void` : free check
<br> </br>
- <h3>Not Recommended</h3>
  
  - `init_board(board: Int32Array) => Board` : initialize board
  - `init_check(board: Board, x: number, y: number) => Check` : initialize check

## First way to use
```bash
npm i @r_seung_h/blom-logic
npm i vite-plugin-wasm
```
<br/>  

### 1. modify vite.config.ts
```ts
import wasm from 'vite-plugin-wasm';
import { defineConfig } from 'vite';

export default defineConfig({
    plugins: [wasm()],
    build: {
        target: 'esnext',
    }
  });
```
build options is for top-level await. (it may be not support old browser)


### 2. Import functions from pkg
this way not use init() function.
```ts
import { init_obj,
    update_check_board, 
    check_win, 
    check_33, 
    check_44, 
} from '@r_seung_h/blom-logic';
```

## Second way to use
this way is recommended for the web (use public path).

### 1. Import wasm and Init module
```js
import init from './pkg/check_logic.js';

const module = await init();
```

### 2. Use functions

```js
import { init_obj,
    update_check_board, 
    check_win, 
    check_33, 
    check_44, 
    check_valid_board, 
    print_address 
} from './pkg/check_logic.js';
```