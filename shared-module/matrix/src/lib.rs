#![no_std]

#[no_mangle]
pub extern "C" fn setup() {
}

#[no_mangle]
pub extern "C" fn setup2() {
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
