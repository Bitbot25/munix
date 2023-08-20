#![cfg(not(test))] // disable tests
#![no_std]  // dont link to the rust standard library
#![no_main] // use our own runtime

// lints
#![warn(clippy::not_unsafe_ptr_arg_deref)]
#![warn(clippy::cast_ptr_alignment)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::unwrap_used)]

#![deny(unreachable_patterns)]
#![deny(unused_must_use)]

use bootloader_api::{BootloaderConfig, BootInfo};
use core::panic::PanicInfo;

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello, World!";
const VGA: *mut u8 = 0xb8000 as *mut _;
const VGA_CYAN: u8 = 0xb;

pub fn kstart(bootinfo: &'static mut BootInfo) -> ! {
    /*if let Some(framebuffer) = bootinfo.framebuffer.as_mut() {
	framebuffer.buffer_mut().fill(255);
    }*/

    //println!("Hello, World!");
    
    loop {}
}

const CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    // 100 KiB stack size
    config.kernel_stack_size = 100 * 1024;
    config
};

bootloader_api::entry_point!(kstart, config = &CONFIG);
