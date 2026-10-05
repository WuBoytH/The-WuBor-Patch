use crate::offsets;

pub fn install() {
    unsafe {
        // Pill Fix for respawn platform
        let _ = skyline::patching::Patch::in_text(offsets::weapon::mariod_drcapsule::RESPAWN_PLATFORM_PILL_PATCH).data(0x14000047u32);
    }
}