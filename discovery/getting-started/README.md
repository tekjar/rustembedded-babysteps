
##### RUN ON QEMU
-----
```
cargo run --example hello --release
```

##### DEBUGGING
-----

```
qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -nographic \
      -semihosting-config enable=on,target=native \
      -gdb tcp::3333 \
      -S \
      -kernel target/thumbv7m-none-eabi/debug/examples/hello
```

* -gdb tcp::3333. This tells QEMU to wait for a GDB connection on TCP port 3333.

* -S. This tells QEMU to freeze the machine at startup. Without this the program would have reached the end of main before we had a chance to launch the debugger!

```
arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/examples/hello
target remote :3333
break main
continue
```