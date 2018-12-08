#### FLASHING AND DEBUGGING
----

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/f3-try

target remote :3333
load
break main
continue
```

Set the runner in `.cargo/config` to automate this process with `cargo run`

```
[target.thumbv7em-none-eabihf]
runner = "arm-none-eabi-gdb -q -x openocd.gdb"
```

Run itmdump in the same folder as openocd for debugging
```
itmdump -F -f itm.txt
```

#### USEFUL GDB COMMANDS
----

```
layout src
tui disable
monitor reset halt # resets the uC but previous breakpoints, ram won't be touched
step/stepi   # stepi will print the statement, the line number and the address of the instruction the processor will execute next.
next         # continues until next break point
print x
print &x
info locals
info args    # args of a function
backtrace
finish       # resumes the program execution and stops it again right after the program returns from the current function
layout asm
break rust_begin_unwind
```


#### TROUBLESHOOT
----

Examination failed, GDB will be halted. Polling again in 100ms

solution: hold reset button, run the above openocd command, release reset button

[reference](https://github.com/japaric/f3/issues/84)

----