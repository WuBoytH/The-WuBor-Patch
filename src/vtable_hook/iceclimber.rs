use crate::offsets;

pub fn install() {
    unsafe {
        // Patches out disabling grabs
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::DISABLE_GRAB_PATCH_1).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::DISABLE_GRAB_PATCH_2).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::DISABLE_GRAB_PATCH_3).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::DISABLE_GRAB_PATCH_4).nop();

        // No more cheering or crying.
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::CHEER_CRY_PATCH_1).data(0x140000A7_u32);
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::CHEER_CRY_PATCH_2).data(0x1400060C_u32);
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::CHEER_CRY_PATCH_3).nop();
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::CHEER_CRY_PATCH_4).data(0x1400049E_u32);

        // A special patch, to see who's the fastest.
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::SPEED_PATCH_1).data(0x140000E1_u32);
        let _ = skyline::patching::Patch::in_text(offsets::fighter::iceclimber::SPEED_PATCH_2).data(0x2A1F03F6_u32);
    }
}
