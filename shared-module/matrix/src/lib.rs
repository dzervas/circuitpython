#![no_std]

#[allow(non_camel_case_types)]
mod mp_basic_types;
#[allow(non_camel_case_types)]
mod mp_obj;

#[no_mangle]
pub extern "C" fn shared_module_matrix_example() -> mp_obj::mp_obj_t {
    unsafe { mp_obj::mp_obj_new_int_from_uint(42) }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
