# README

Add tools for managing, running wasm and wasm components file.
```
$ cargo install wasm-tools
$ cargo install wasmtime
$ cargo install cargo-component
```
Add - wasm32 compiler

```
$ rustup target add wasm32-wasi
```

Creat a new component
```
cargo component new command --command
```

Check generated wasm file 

```
$ wasm-tools component wit hello.wasm 
```

Run with  wasmtime a wasm file

```
wasmtime run --wasm component-model command.wasm 1 2 add
```

Compose wasm component from other wasm components
```
wasm-tools compose calculator/target/wasm32-wasi/release/calculator.wasm -d adder/target/wasm32-wasi/release/adder.wasm -o composed.wasm
wasm-tools compose command/target/wasm32-wasi/release/command.wasm -d composed.wasm -o command.wasm
```



