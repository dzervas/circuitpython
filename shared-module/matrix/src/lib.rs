#![no_std]

#[no_mangle]
pub extern "C" fn shared_module_matrix_example() {
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
