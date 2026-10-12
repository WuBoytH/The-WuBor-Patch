use wubor_utils::offsets;

pub fn install() {
    unsafe {
        // Removes a Psyche Up check
        skyline::patching::Patch::in_text(offsets::fighter::brave::PSYCHE_UP_CHECK_PATCH).data(0x14000010u32);
    }
}
