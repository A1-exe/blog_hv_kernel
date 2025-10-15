#![no_std]
#![no_main]
#![allow(static_mut_refs)]

extern crate alloc;

use kernel::{println, serial_println as sprintln};
use bootloader_api::{entry_point, 
    info::BootInfo, 
    config::{ BootloaderConfig, Mapping }
};
use core::panic::PanicInfo;

use kernel::BOOT_INFO;

// use x86_64::structures::paging::{Page, PageTable};
use x86_64::VirtAddr;

use alloc::vec;
use kernel::task::executor::Executor;
use kernel::task::keyboard;
// use blog_os::task::simple_executor::SimpleExecutor;
use kernel::task::Task;


pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 90 * 1024;
    config.mappings.physical_memory = Some(Mapping::FixedAddress(0x180_0000_0000));
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    sprintln!("!!!!!!!!! KERNEL STARTED !!!!!!!!");
    unsafe { BOOT_INFO = Some(boot_info) };
    // sprintln!("Memory starts at: {:#x?}", unsafe { BOOT_INFO.as_deref_mut().unwrap() }.memory_regions.as_ref());


    use kernel::allocator;
    use kernel::memory::{self, BootInfoFrameAllocator};

    kernel::init();

    let boot_info = unsafe { BOOT_INFO.as_deref_mut().unwrap() };
    let phys_mem_offset = VirtAddr::new(*boot_info.physical_memory_offset.as_ref().expect("physical_memory_offset not provided"));
    println!("Physical memory offset: {:#x}", phys_mem_offset);
    println!("Kernel address: {:#x}", boot_info.kernel_addr);
    println!("Kernel end address: {:#x}", boot_info.kernel_addr + boot_info.kernel_len);
    println!("Kernel image offset: {:#x}", boot_info.kernel_image_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let v = vec![1, 2, 3];
    println!("vec at {:p}", v.as_ptr());
    for n in v {
        println!("vec value: {}", n);
    }

    println!("\n== CHECKING CPU ==");
    // Check if AMD-V is enabled
    let cpuid = raw_cpuid::CpuId::new();
    if let Some(vi) = cpuid.get_vendor_info() {
        if vi.as_str() == "AuthenticAMD" {
            println!("AMD CPU detected");

            if let Some(_) = cpuid.get_svm_info(){
                println!("AMD-V is enabled");
            } else {
                println!("AMD-V is not enabled");
            }
        } else {
            println!("Non-AMD CPU detected");
        }
    }
    println!("== COMPLETE ==\n");

    // sleep_ms(1000); // Sleep for 1 seconds before re-initializing the logger.

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[allow(unused)]
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
    kernel::serial_println!("{}", _info);
    kernel::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kernel::test_panic_handler(_info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}