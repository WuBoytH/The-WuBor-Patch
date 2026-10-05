use crate::offsets;

pub fn install() {
    unsafe {
        // Disables Foresight
        skyline::patching::Patch::in_text(offsets::fighter::elight::FORESIGHT_PATCH_1).nop();
        skyline::patching::Patch::in_text(offsets::fighter::elight::FORESIGHT_PATCH_2).data(0x140000ACu32);
    }
}