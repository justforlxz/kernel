#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kernel::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");
  kernel::init();

  #[cfg(test)]
  test_main();
  println!("It did not crash!");
  loop {}
}

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  println!("{}", _info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  kernel::test_panic_handler(info)
}
