#![no_std]
#![no_main]

use core::fmt::Write;
use bootloader_x86_64_common::framebuffer::FrameBufferWriter;
use bootloader_api::{entry_point, BootInfo};

use core::panic::PanicInfo;

use core::arch::asm;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::serial_println!("!!!!!!!!! KERNEL STARTED !!!!!!!!");

    let framebuffer = boot_info.framebuffer.as_mut()
        .expect("Framebuffer not found");

    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    // For reusing bootloader's logger.
    // bootloader_x86_64_common::init_logger(
    //     buffer,
    //     info,
    //     bootloader_boot_config::LevelFilter::Info,
    //     true,
    //     true
    // );
    
    // log::info!("Logger re-initialized.");

    // For raw writes reusing the buffer.

    sleep_ms(5000); // Sleep for 5 seconds before re-initializing the logger.

    let mut writer = FrameBufferWriter::new(buffer, info);
    writeln!(writer, "Logger re-initialized.").unwrap();

    loop {}
}

/// Approximate CPU frequency in Hz.
/// ⚠️ Replace this with the actual frequency of your target machine or calibrate dynamically.
const CPU_FREQUENCY_HZ: u64 = 3_000_000_000; // 3.0 GHz

/// Sleep for `ms` milliseconds using a busy loop on the TSC.
pub fn sleep_ms(ms: u64) {
    use core::arch::x86_64::_rdtsc;

    // Convert ms to CPU cycles
    let cycles_to_wait = (CPU_FREQUENCY_HZ / 1_000) * ms;
    let start = unsafe { _rdtsc() };

    while unsafe { _rdtsc() } - start < cycles_to_wait {
        // Hint to CPU to reduce power usage while spinning
        core::hint::spin_loop();
    }
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", _info);
    kernel::hlt_loop()
}