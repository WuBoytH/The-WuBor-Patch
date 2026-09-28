use crate::offsets;

pub fn install() {
    unsafe {
        // Related to Crafting Table auto-respawn
        // Removes the link event setting the auto respawn timer
        skyline::patching::Patch::in_text(offsets::fighter::pickel::CRAFTING_TABLE_AUTO_RESPAWN_LINK_EVENT_PATCH).data(0x140003ACu32);
        // Disables the count_down_int for the auto respawn timer
        skyline::patching::Patch::in_text(offsets::fighter::pickel::CRAFTING_TABLE_AUTO_RESPAWN_COUNT_DOWN_PATCH).nop();
        // Skips the auto respawn create table call
        skyline::patching::Patch::in_text(offsets::fighter::pickel::CRAFTING_TABLE_AUTO_RESPAWN_CREATE_PATCH).data(0x14000005u32);
    }
}