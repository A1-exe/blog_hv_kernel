#![no_std]

pub mod serial;

// Halt the CPU until the next interrupt arrives.
// This allows the CPU to enter a sleep state in which it consumes much less energy
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
