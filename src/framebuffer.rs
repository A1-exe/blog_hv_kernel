use spin::Mutex;
use bootloader_x86_64_common::framebuffer::FrameBufferWriter;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WRITER: Mutex<FrameBufferWriter> = {
        let framebuffer = unsafe { crate::BOOT_INFO.as_deref_mut().unwrap().framebuffer.as_mut()
            .expect("Framebuffer not found") };
        
        let info = framebuffer.info();
        let buffer = framebuffer.buffer_mut();

        Mutex::new(FrameBufferWriter::new(buffer, info))
    };
}

use core::fmt::{self, Write};

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;

    // Fix print deadlock
    interrupts::without_interrupts(|| WRITER.lock().write_fmt(args).unwrap());
}

#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
  () => ($crate::print!("\n"));
  ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
