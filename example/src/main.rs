use fbink_sys::*;
use std::{
    ffi::{c_char, CStr, CString},
    mem::MaybeUninit,
    process::exit,
};

fn main() {
    println!("Starting fbink-sys example");

    let mut fbink_cfg: FBInkConfig = unsafe { std::mem::transmute([0u8; std::mem::size_of::<FBInkConfig>()]) };
    println!("fbink_cfg default state: {:?}", fbink_cfg);
    fbink_cfg.is_flashing = true;
    fbink_cfg.is_centered = true;
    fbink_cfg.is_halfway = true;

    let fbfd: ::std::os::raw::c_int = 0;

    unsafe {
        if fbink_init(fbfd, &fbink_cfg) < 0 {
            println!("Failed to init fbink");
            exit(1);
        }
    }

    let cls_rect: FBInkRect = FBInkRect {
        left: 0,
        top: 0,
        width: 0,
        height: 0,
    };
    unsafe {
        if fbink_cls(fbfd, &fbink_cfg, &cls_rect, false) < 0 {
            println!("Failed to clear screen");
            exit(1);
        } else {
            println!("Cleared the screen");
        }
    };
    unsafe { fbink_wait_for_complete(fbfd, LAST_MARKER) };

    let mut fbinkOT_cfg: FBInkOTConfig = unsafe { std::mem::transmute([0u8; std::mem::size_of::<FBInkOTConfig>()]) };
    fbinkOT_cfg.is_centered = true;
    fbinkOT_cfg.is_formatted = true;
    fbinkOT_cfg.size_pt = 20.0;

    let mut ot_fit: FBInkOTFit = unsafe { std::mem::transmute([0u8; std::mem::size_of::<FBInkOTFit>()]) };
    let mut to_print = CString::new("fbink-sys demo")
        .expect("CString::new failed")
        .as_ptr();

    unsafe {
        if fbink_print_ot(fbfd, to_print, &fbinkOT_cfg, &fbink_cfg, &mut ot_fit) < 0 {
            println!("Failed to print to fbink");
            exit(1);
        }
    }

    unsafe { fbink_wait_for_complete(fbfd, LAST_MARKER) };
    exit(0);
}
