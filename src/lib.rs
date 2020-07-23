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
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size / 4,
            temp = out(reg) _,
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
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size / 8,
            temp = out(reg) _,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_128(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    movdqa {temp}, [{source} + {counter:r}]",
            "    movdqa [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 16",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            temp = out(xmm_reg) _,
            options(nostack),
        );
    }
}

#[cfg(target_feature = "avx")]
pub fn memcpy_mov_256(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    vmovdqa {temp}, [{source} + {counter:r}]",
            "    vmovdqa [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 32",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            temp = out(ymm_reg) _,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_64_pl(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    mov {temp0:r}, [{source} + {counter:r}]",
            "    mov {temp1:r}, [{source} + {counter:r} + 8]",
            "    mov {temp2:r}, [{source} + {counter:r} + 16]",
            "    mov {temp3:r}, [{source} + {counter:r} + 24]",
            "    mov {temp4:r}, [{source} + {counter:r} + 32]",
            "    mov {temp5:r}, [{source} + {counter:r} + 40]",
            "    mov {temp6:r}, [{source} + {counter:r} + 48]",
            "    mov {temp7:r}, [{source} + {counter:r} + 56]",
            "    mov [{destination} + {counter:r}], {temp0:r}",
            "    mov [{destination} + {counter:r} + 8], {temp1:r}",
            "    mov [{destination} + {counter:r} + 16], {temp2:r}",
            "    mov [{destination} + {counter:r} + 24], {temp3:r}",
            "    mov [{destination} + {counter:r} + 32], {temp4:r}",
            "    mov [{destination} + {counter:r} + 40], {temp5:r}",
            "    mov [{destination} + {counter:r} + 48], {temp6:r}",
            "    mov [{destination} + {counter:r} + 56], {temp7:r}",
            "    add {counter:r}, 64",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            temp0 = out(reg) _,
            temp1 = out(reg) _,
            temp2 = out(reg) _,
            temp3 = out(reg) _,
            temp4 = out(reg) _,
            temp5 = out(reg) _,
            temp6 = out(reg) _,
            temp7 = out(reg) _,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_128_pl(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    movdqa xmm0, [{source} + {counter:r}]",
            "    movdqa xmm1, [{source} + {counter:r} + 16]",
            "    movdqa xmm2, [{source} + {counter:r} + 32]",
            "    movdqa xmm3, [{source} + {counter:r} + 48]",
            "    movdqa xmm4, [{source} + {counter:r} + 64]",
            "    movdqa xmm5, [{source} + {counter:r} + 80]",
            "    movdqa xmm6, [{source} + {counter:r} + 96]",
            "    movdqa xmm7, [{source} + {counter:r} + 112]",
            "    movdqa [{destination} + {counter:r}], xmm0",
            "    movdqa [{destination} + {counter:r} + 16], xmm1",
            "    movdqa [{destination} + {counter:r} + 32], xmm2",
            "    movdqa [{destination} + {counter:r} + 48], xmm3",
            "    movdqa [{destination} + {counter:r} + 64], xmm4",
            "    movdqa [{destination} + {counter:r} + 80], xmm5",
            "    movdqa [{destination} + {counter:r} + 96], xmm6",
            "    movdqa [{destination} + {counter:r} + 112], xmm7",
            "    add {counter:r}, 128",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            out("xmm0") _,
            out("xmm1") _,
            out("xmm2") _,
            out("xmm3") _,
            out("xmm4") _,
            out("xmm5") _,
            out("xmm6") _,
            out("xmm7") _,
            options(nostack),
        );
    }
}

#[cfg(target_feature = "avx")]
pub fn memcpy_mov_256_pl(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    vmovdqa ymm0, [{source} + {counter:r}]",
            "    vmovdqa ymm1, [{source} + {counter:r} + 32]",
            "    vmovdqa ymm2, [{source} + {counter:r} + 64]",
            "    vmovdqa ymm3, [{source} + {counter:r} + 96]",
            "    vmovdqa ymm4, [{source} + {counter:r} + 128]",
            "    vmovdqa ymm5, [{source} + {counter:r} + 160]",
            "    vmovdqa ymm6, [{source} + {counter:r} + 192]",
            "    vmovdqa ymm7, [{source} + {counter:r} + 224]",
            "    vmovdqa [{destination} + {counter:r}], ymm0",
            "    vmovdqa [{destination} + {counter:r} + 32], ymm1",
            "    vmovdqa [{destination} + {counter:r} + 64], ymm2",
            "    vmovdqa [{destination} + {counter:r} + 96], ymm3",
            "    vmovdqa [{destination} + {counter:r} + 128], ymm4",
            "    vmovdqa [{destination} + {counter:r} + 160], ymm5",
            "    vmovdqa [{destination} + {counter:r} + 192], ymm6",
            "    vmovdqa [{destination} + {counter:r} + 224], ymm7",
            "    add {counter:r}, 256",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            // out("ymm0") _,
            // out("ymm1") _,
            // out("ymm2") _,
            // out("ymm3") _,
            // out("ymm4") _,
            // out("ymm5") _,
            // out("ymm6") _,
            // out("ymm7") _,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_64_nt(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    mov {temp:r}, [{source} + {counter:r} * 8]",
            "    movnti [{destination} + {counter:r} * 8], {temp:r}",
            "    inc {counter:r}",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size / 8,
            temp = out(reg) _,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_128_nt(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    movdqa {temp}, [{source} + {counter:r}]",
            "    movntdq [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 16",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            temp = out(xmm_reg) _,
            options(nostack),
        );
    }
}

#[cfg(target_feature = "avx")]
pub fn memcpy_mov_256_nt(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    vmovdqa {temp}, [{source} + {counter:r}]",
            "    vmovntdq [{destination} + {counter:r}], {temp}",
            "    add {counter:r}, 32",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            temp = out(ymm_reg) _,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_64_nt_pl(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "2:",
            "    mov {temp0:r}, [{source} + {counter:r}]",
            "    mov {temp1:r}, [{source} + {counter:r} + 8]",
            "    mov {temp2:r}, [{source} + {counter:r} + 16]",
            "    mov {temp3:r}, [{source} + {counter:r} + 24]",
            "    mov {temp4:r}, [{source} + {counter:r} + 32]",
            "    mov {temp5:r}, [{source} + {counter:r} + 40]",
            "    mov {temp6:r}, [{source} + {counter:r} + 48]",
            "    mov {temp7:r}, [{source} + {counter:r} + 56]",
            "    movnti [{destination} + {counter:r}], {temp0:r}",
            "    movnti [{destination} + {counter:r} + 8], {temp1:r}",
            "    movnti [{destination} + {counter:r} + 16], {temp2:r}",
            "    movnti [{destination} + {counter:r} + 24], {temp3:r}",
            "    movnti [{destination} + {counter:r} + 32], {temp4:r}",
            "    movnti [{destination} + {counter:r} + 40], {temp5:r}",
            "    movnti [{destination} + {counter:r} + 48], {temp6:r}",
            "    movnti [{destination} + {counter:r} + 56], {temp7:r}",
            "    add {counter:r}, 64",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            temp0 = out(reg) _,
            temp1 = out(reg) _,
            temp2 = out(reg) _,
            temp3 = out(reg) _,
            temp4 = out(reg) _,
            temp5 = out(reg) _,
            temp6 = out(reg) _,
            temp7 = out(reg) _,
            source = in(reg) source,
            options(nostack),
        );
    }
}

pub fn memcpy_mov_128_nt_pl(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    movdqa xmm0, [{source} + {counter:r}]",
            "    movdqa xmm1, [{source} + {counter:r} + 16]",
            "    movdqa xmm2, [{source} + {counter:r} + 32]",
            "    movdqa xmm3, [{source} + {counter:r} + 48]",
            "    movdqa xmm4, [{source} + {counter:r} + 64]",
            "    movdqa xmm5, [{source} + {counter:r} + 80]",
            "    movdqa xmm6, [{source} + {counter:r} + 96]",
            "    movdqa xmm7, [{source} + {counter:r} + 112]",
            "    movntdq [{destination} + {counter:r}], xmm0",
            "    movntdq [{destination} + {counter:r} + 16], xmm1",
            "    movntdq [{destination} + {counter:r} + 32], xmm2",
            "    movntdq [{destination} + {counter:r} + 48], xmm3",
            "    movntdq [{destination} + {counter:r} + 64], xmm4",
            "    movntdq [{destination} + {counter:r} + 80], xmm5",
            "    movntdq [{destination} + {counter:r} + 96], xmm6",
            "    movntdq [{destination} + {counter:r} + 112], xmm7",
            "    add {counter:r}, 128",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            out("xmm0") _,
            out("xmm1") _,
            out("xmm2") _,
            out("xmm3") _,
            out("xmm4") _,
            out("xmm5") _,
            out("xmm6") _,
            out("xmm7") _,
            options(nostack),
        );
    }
}

#[cfg(target_feature = "avx")]
pub fn memcpy_mov_256_nt_pl(size: usize, source: *mut u8, destination: *mut u8) {
    unsafe {
        asm!(
            "    vzeroall",
            "2:",
            "    vmovdqa ymm0, [{source} + {counter:r}]",
            "    vmovdqa ymm1, [{source} + {counter:r} + 32]",
            "    vmovdqa ymm2, [{source} + {counter:r} + 64]",
            "    vmovdqa ymm3, [{source} + {counter:r} + 96]",
            "    vmovdqa ymm4, [{source} + {counter:r} + 128]",
            "    vmovdqa ymm5, [{source} + {counter:r} + 160]",
            "    vmovdqa ymm6, [{source} + {counter:r} + 192]",
            "    vmovdqa ymm7, [{source} + {counter:r} + 224]",
            "    vmovntdq [{destination} + {counter:r}], ymm0",
            "    vmovntdq [{destination} + {counter:r} + 32], ymm1",
            "    vmovntdq [{destination} + {counter:r} + 64], ymm2",
            "    vmovntdq [{destination} + {counter:r} + 96], ymm3",
            "    vmovntdq [{destination} + {counter:r} + 128], ymm4",
            "    vmovntdq [{destination} + {counter:r} + 160], ymm5",
            "    vmovntdq [{destination} + {counter:r} + 192], ymm6",
            "    vmovntdq [{destination} + {counter:r} + 224], ymm7",
            "    add {counter:r}, 256",
            "    cmp {counter:r}, {size}",
            "    jne 2b",
            source = in(reg) source,
            destination = in(reg) destination,
            counter = inout(reg) 0 => _,
            size = in(reg) size,
            // out("ymm0") _,
            // out("ymm1") _,
            // out("ymm2") _,
            // out("ymm3") _,
            // out("ymm4") _,
            // out("ymm5") _,
            // out("ymm6") _,
            // out("ymm7") _,
            options(nostack),
        );
    }
}

pub fn memcpy_rep_movsb(size: usize, source: *mut u8, destination: *mut u8) {
    let mut _size = size;
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

pub fn memcpy_rep_movsq(size: usize, source: *mut u8, destination: *mut u8) {
    let mut _size = size / 8;
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
