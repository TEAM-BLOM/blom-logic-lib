# Overview

## Current
check win logic is implemented in rust, and the web app is to test the pkg.

## Folder
1. `lib-cpp` : tried to implement a library in c++;  
    using emsdk to compile to wasm, but failed to use it in js

2. `lib-rust` : tried to implement a library in rust;  
    refer to the [lib-rust/README.md](./lib-rust/README.md) for more details

3. `pkg` : built a wasm, js package using rust
   
4. `testWeb` : a web app to test the pkg  
    just run `live-server` in the `testWeb` folder to preview (using `index.html`)

## TODO
1. implement a check 33 and check 44 logic in rust.