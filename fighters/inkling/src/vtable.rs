// use super::*;
use wubor_utils::offsets;

pub fn install() {
    unsafe {
        let _ = skyline::patching::Patch::in_text(offsets::fighter::inkling::vtable::REVERSE_INK_DETECTION).data(0x54002681_u32);
    }
}