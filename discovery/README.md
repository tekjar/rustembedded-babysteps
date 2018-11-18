##### FREQUENTLY USED COMMANDS
-------

```
cargo generate --git https://github.com/rust-embedded/cortex-m-quickstart
```

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

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
* 

##### TROUBLE SHOOT
--------

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

Examination failed, GDB will be halted. Polling again in 100ms

solution: hold reset button, run the above openocd command, release reset button

[reference](https://github.com/japaric/f3/issues/84)

----
