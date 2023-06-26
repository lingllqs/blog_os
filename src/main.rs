#![no_std] // 不链接 Rust 标准库
#![no_main] // 禁用所有 Rust 层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

// 因为链接器会寻找一个名为 `_start` 的函数，所以这个函数就是入口点
// 默认命名为 `_start`
#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    println!("Hello World!{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// 测试函数
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

/// 这个函数将在 panic 时被调用

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// #[allow(dead_code)]
// fn test_02() {
//     println!("Hello World{}", "!");
//     panic!("Some panic message");
// }
//
// #[allow(dead_code)]
// fn test_01() {
//     use core::fmt::Write;
//     vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
//     write!(
//         vga_buffer::WRITER.lock(),
//         ", some numbers: {} {}",
//         42,
//         1.337
//     )
//     .unwrap();
//     // vga_buffer::print_something();
// }
//
// #[allow(dead_code)]
// fn test() {
//     static HELLO: &[u8] = b"Hello World!";
//     let vga_buffer = 0xb8000 as *mut u8;
//
//     for (i, &byte) in HELLO.iter().enumerate() {
//         unsafe {
//             // 每个字符都有字形和样式,一个字符用两个字节表示
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // 0x0b 淡青色
//         }
//     }
// }
