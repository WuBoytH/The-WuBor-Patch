#![allow(static_mut_refs)]

use std::sync::atomic::{Ordering, AtomicBool};
use crate::is_on_ryujinx;

unsafe fn calc_nnsdk_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x3747b7c, inline)]
unsafe fn run_scene_update(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}

#[skyline::hook(replace = OFFSET1)]
unsafe fn set_interval_1(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn set_interval_2(ctx: &mut skyline::hooks::InlineCtx) {
    *ctx.registers[8].x.as_mut() = 0;
}

static mut RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x3810a64, inline)]
unsafe fn vsync_count_thread(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

pub fn install() {
    if !is_on_ryujinx() {
        unsafe {
            OFFSET1 = calc_nnsdk_offset() + 0x429d60;
            OFFSET2 = calc_nnsdk_offset() + 0x26e94;
        }
    
        skyline::install_hooks!(
            set_interval_1,
            set_interval_2,
            run_scene_update,
            vsync_count_thread,
        );
    }
}