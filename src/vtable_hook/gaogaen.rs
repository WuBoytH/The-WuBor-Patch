use wubor_utils::offsets;

pub fn install() {
    unsafe {
        // Skips to the end of the Revenge check after changing statuses
        skyline::patching::Patch::in_text(offsets::fighter::gaogaen::REVENGE_CHECK_SKIP_PATCH).data(0x140000D1u32);
    }
}
