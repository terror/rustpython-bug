### rustpython-bug

```bash
git clone https://github.com/terror/rustpython-bug
cd rustpython-bug
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/rustpython_bug.wasm --out-dir www
python3 -m http.server --directory www --bind 0.0.0.0
```

Open localhost:8000, open the browser console, see this:

```
rustpython_bug.js:108 ImportError: Cannot import builtin module _thread

imports.wbg.__wbg_error_8fd01a064902a0b0 @ rustpython_bug.js:108
rustpython_bug_bg.wasm:0x6e7a82 Uncaught RuntimeError: unreachable
    at __rust_start_panic (rustpython_bug_bg.wasm:0x6e7a82)
    at rust_panic (rustpython_bug_bg.wasm:0x6e48c0)
    at std::panicking::rust_panic_with_hook::h101a042c5db45a5c (rustpython_bug_bg.wasm:0x6a992d)
    at std::panicking::begin_panic_handler::{{closure}}::hbf97ac24a28fe382 (rustpython_bug_bg.wasm:0x6c0a29)
    at std::sys_common::backtrace::__rust_end_short_backtrace::h7e7feb233dcd044b (rustpython_bug_bg.wasm:0x6e62a2)
    at rust_begin_unwind (rustpython_bug_bg.wasm:0x6e261b)
    at core::panicking::panic_fmt::h36fb5bc602204efa (rustpython_bug_bg.wasm:0x6e28a0)
    at rustpython_vm::vm::vm_object::<impl rustpython_vm::vm::VirtualMachine>::_py_panic_failed::h9986ba68918f09e2 (rustpython_bug_bg.wasm:0x6c0abe)
    at rustpython_vm::vm::VirtualMachine::initialize::hebb244e92db09346 (rustpython_bug_bg.wasm:0x4fc07e)
    at rustpython_vm::vm::interpreter::Interpreter::without_stdlib::hd5ed2c2016030e26 (rustpython_bug_bg.wasm:0x6c718b)
```
