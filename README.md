# alt-branch-diff
Alt linux branch difference that uses BaseALT experimental API

Tested on: _Alt Regular JeOS (December 2025)_

## Runtime dependencies
```zsh
alt-branch-diff $ ldd ./target/release/cli
    linux-vdso.so.1
    libssl.so.3
    libcrypto.so.3
    libgcc_s.so.1
    libm.so.6
    libc.so.6
    /lib64/ld-linux-x86-64.so.2
    libz.so.1
```

## Build requierements
- gcc
- libssl-devel
- Rust toolchain (cargo, rustc)

## Compile
```bash
cd alt-branch-diff/
cargo build --release
```

Then you can find the binary in `alt-branch-diff/target/release/` named `cli`