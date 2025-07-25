#![no_std]

use core::iter::Iterator;
static mut CURSOR_POS: usize = 0;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

pub fn print(s: &str) {
    let vga_buffer = 0xb8000 as *mut u8;
    
    for byte in s.bytes() {
        unsafe {
            if byte == b'\n' {
                CURSOR_POS = (CURSOR_POS / VGA_WIDTH + 1) * VGA_WIDTH;
            } else {
                if CURSOR_POS < VGA_WIDTH * VGA_HEIGHT {
                    *vga_buffer.offset(CURSOR_POS as isize * 2) = byte;
                    *vga_buffer.offset(CURSOR_POS as isize * 2 + 1) = 0x0f;
                    CURSOR_POS += 1;
                }
            }
        }
    }
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *dest.add(i) = *src.add(i);
        }
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        for i in 0..n {
            *s.add(i) = c as u8;
        }
    }
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        if src < dest as *const u8 {
            for i in (0..n).rev() {
                *dest.add(i) = *src.add(i);
            }
        } else {
            for i in 0..n {
                *dest.add(i) = *src.add(i);
            }
        }
    }
    dest
}