use kurbo::{BezPath, Cap, Join, Stroke, StrokeOpts};
use std::alloc::{alloc, dealloc, Layout};
use std::slice;
use std::str;

static mut LAST_OUT: Vec<u8> = Vec::new();

#[no_mangle]
pub extern "C" fn alloc_buf(len: usize) -> *mut u8 {
    if len == 0 {
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(len, 1).unwrap();
    unsafe { alloc(layout) }
}

#[no_mangle]
pub extern "C" fn dealloc_buf(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    let layout = Layout::from_size_align(len, 1).unwrap();
    unsafe { dealloc(ptr, layout) };
}

#[no_mangle]
pub extern "C" fn out_ptr() -> *const u8 {
    unsafe { LAST_OUT.as_ptr() }
}

#[no_mangle]
pub extern "C" fn out_len() -> usize {
    unsafe { LAST_OUT.len() }
}

/// 返回 0 表示成功，非 0 表示失败
#[no_mangle]
pub extern "C" fn outline_svg(ptr: *const u8, len: usize, width: f64) -> i32 {
    if !width.is_finite() || width <= 0.0 || ptr.is_null() || len == 0 {
        return -1;
    }
    let bytes = unsafe { slice::from_raw_parts(ptr, len) };
    let svg_d = match str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let bp = match BezPath::from_svg(svg_d) {
        Ok(b) => b,
        Err(_) => return -3,
    };
    if bp.elements().is_empty() {
        unsafe { LAST_OUT.clear(); }
        return 0;
    }

    let style = Stroke::new(width)
        .with_caps(Cap::Round)
        .with_join(Join::Round)
        .with_miter_limit(4.0);
    let stroked = kurbo::stroke(bp.iter(), &style, &StrokeOpts::default(), 1e-3);
    let out_str = stroked.to_svg();

    unsafe {
        LAST_OUT = out_str.into_bytes();
    }
    0
}
