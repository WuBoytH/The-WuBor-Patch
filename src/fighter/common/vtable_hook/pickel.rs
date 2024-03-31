pub fn install() {
    // Related to Crafting Table auto-respawn
    // Removes the link event setting the auto respawn timer
    skyline::patching::Patch::in_text(0xf0c20c).data(0x140003ACu32);
    // Disables the count_down_int for the auto respawn timer
    skyline::patching::Patch::in_text(0xf088c8).nop();
    // Skips the auto respawn create table call
    skyline::patching::Patch::in_text(0xf088cc).data(0x14000005u32);
}