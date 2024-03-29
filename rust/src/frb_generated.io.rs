// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.14.

// Section: imports

use super::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: dart2rust

impl CstDecode<String> for *mut wire_cst_list_prim_u_8_strict {
    fn cst_decode(self) -> String {
        let vec: Vec<u8> = self.cst_decode();
        String::from_utf8(vec).unwrap()
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_loose {
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl CstDecode<Vec<u8>> for *mut wire_cst_list_prim_u_8_strict {
    fn cst_decode(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::for_generated::box_from_leak_ptr(self);
            flutter_rust_bridge::for_generated::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_image_signing_dart_fn_deliver_output(
    call_id: i32,
    ptr_: *mut u8,
    rust_vec_len_: i32,
    data_len_: i32,
) {
    let message = unsafe {
        flutter_rust_bridge::for_generated::Dart2RustMessageSse::from_wire(
            ptr_,
            rust_vec_len_,
            data_len_,
        )
    };
    FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_handle_output(call_id, message)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_image_signing_wire_sign(
    certificate_contents: *mut wire_cst_list_prim_u_8_loose,
    private_key_contents: *mut wire_cst_list_prim_u_8_loose,
    image_contents: *mut wire_cst_list_prim_u_8_loose,
) -> flutter_rust_bridge::for_generated::WireSyncRust2DartDco {
    wire_sign_impl(certificate_contents, private_key_contents, image_contents)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_image_signing_cst_new_list_prim_u_8_loose(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_loose {
    let ans = wire_cst_list_prim_u_8_loose {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn frbgen_flutter_rust_image_signing_cst_new_list_prim_u_8_strict(
    len: i32,
) -> *mut wire_cst_list_prim_u_8_strict {
    let ans = wire_cst_list_prim_u_8_strict {
        ptr: flutter_rust_bridge::for_generated::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::for_generated::new_leak_box_ptr(ans)
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_loose {
    ptr: *mut u8,
    len: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct wire_cst_list_prim_u_8_strict {
    ptr: *mut u8,
    len: i32,
}
