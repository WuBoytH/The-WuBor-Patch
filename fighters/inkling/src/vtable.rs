// use super::*;

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x6b3450).data(0x54002681_u32);
}