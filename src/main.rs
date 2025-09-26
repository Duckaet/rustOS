#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo;

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuEmitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuEmitCode){
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]){
    serial_println!("RUnning {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuEmitCode::Success);
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();

     loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion(){
    serial_print!("trivial assertion..");
    assert_eq!(1,1);
    serial_println!("[ok]");
}

