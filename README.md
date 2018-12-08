# rustembedded-babysteps

#### STARTING A NEW PROJECT
------

```
cargo install cargo-generate
rustup component add llvm-tools-preview
cargo install cargo-binutils --vers 0.1.4
cargo install itm --vers 0.3.1

cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

##### FREQUENTLY USED COMMANDS
-------

```
cargo readobj --bin app -- -file-headers
```

```
cargo size --bin app --release -- -A
```

```
cargo objdump --bin app --release -- -disassemble -no-show-raw-insn -print-imm-hex
```

##### NOTES
--------
* ELF files contain metadata like debug information so their size on disk does not accurately reflect the space the program will occupy when flashed on a device. Always use cargo-size to check how big a binary really is