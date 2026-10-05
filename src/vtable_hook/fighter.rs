use crate::offsets;

pub fn install() {
    unsafe {
        // Sets jostle team for all fighters to 0
        skyline::patching::Patch::in_text(offsets::fighter::common::JOSTLE_TEAM_PATCH).data(0x52800001u32);
    }
}