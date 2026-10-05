use crate::offsets;

pub fn install() {
    unsafe {
        // Always start a match with Levin Sword
        let _ = skyline::patching::Patch::in_text(offsets::fighter::reflet::LEVIN_SWORD_START_PATCH).nop();

        // Fixes an issue causing Robin to be un-trippable after using Thoron
        let _ = skyline::patching::Patch::in_text(offsets::fighter::reflet::THORON_TRIP_FIX_PATCH).nop();
    }
}