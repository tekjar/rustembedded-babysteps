#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

use cortex_m::asm;
use cortex_m_rt::entry;

fn itm() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();
    p.ITM
}

#[entry]
fn main() -> ! {
    let x = 42;
    let mut itm = itm();

    iprintln!(&mut itm.stim[0], "Hello World!!");

    panic!("Hello world");
    // infinite loop; just so we don't leave this stack frame
    loop {}
}



