pub fn install() {
    // Forces the common mining pattern
    skyline::patching::Patch::in_text(0xf1076c).data(0x14000012u32);

    // Skips the check for what tool to mine with
    skyline::patching::Patch::in_text(0xf0f774).data(0x14000023u32);

    // Forces the mining tool used to be Pickaxe
    skyline::patching::Patch::in_text(0xf0f800).data(0x321F03E1u32);

    // Related to Crafting Table auto-respawn
    // Removes the link event setting the auto respawn timer
    skyline::patching::Patch::in_text(0xf0c1ec).data(0x140003ACu32);
    // Disables the count_down_int for the auto respawn timer
    skyline::patching::Patch::in_text(0xf088a8).nop();
    // Skips the auto respawn create table call
    skyline::patching::Patch::in_text(0xf088ac).data(0x14000005u32);
}