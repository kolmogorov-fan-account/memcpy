#![feature(asm)]

pub fn memcpy_mov_32(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    mov {temp:e}, [{source} + {counter:r} * 4]",
            "    mov [{destination} + {counter:r} * 4], {temp:e}",
            "    inc {counter:r}",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            temp = out(reg) _,
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size / 4,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_64(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    mov {temp:r}, [{source} + {counter:r} * 8]",
            "    mov [{destination} + {counter:r} * 8], {temp:r}",
            "    inc {counter:r}",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            temp = out(reg) _,
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size / 8,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_128(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    movdqa {temp}, [{source} + {counter:r}]",
            "    movdqa [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 16",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            temp = out(xmm_reg) _,
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            options(nostack),
        );
    }
}

#[cfg(target_feature = "avx")]
pub fn memcpy_mov_256(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    vmovdqa {temp}, [{source} + {counter:r}]",
            "    vmovdqa [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 32",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            temp = out(ymm_reg) _,
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_128_nt(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    movdqa {temp}, [{source} + {counter:r}]",
            "    movntdq [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 16",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            temp = out(xmm_reg) _,
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            options(nostack),
        );
    }
}

#[cfg(target_feature = "avx")]
pub fn memcpy_mov_256_nt(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    vmovdqa {temp}, [{source} + {counter:r}]",
            "    vmovntdq [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 32",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            temp = out(ymm_reg) _,
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            options(nostack),
        );
    }
}

pub fn memcpy_rep_movsb(mut _size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "rep movsb",
            in("rsi") source,
            in("rdi") destination,
            inout("rcx") _size,
            options(nostack),
        );
    }
}

pub fn memcpy_rep_movsq(mut _size: usize, source: *mut u8, destination: *mut u8) {
    _size /= 8;
    unsafe {
        asm!(
            "rep movsq",
            in("rsi") source,
            in("rdi") destination,
            inout("rcx") _size,
            options(nostack),
        );
    }
}
