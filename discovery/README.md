

##### TROUBLE SHOOT
--------

```
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

Examination failed, GDB will be halted. Polling again in 100ms

solution: hold reset button, run the above openocd command, release reset button

[reference](https://github.com/japaric/f3/issues/84)

----
