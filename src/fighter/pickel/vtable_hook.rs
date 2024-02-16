pub fn install() {
    // Forces the common mining pattern
    skyline::patching::Patch::in_text(0xf1078c).data(0x14000012u32);

    // Skips the check for what tool to mine with
    skyline::patching::Patch::in_text(0xf0f794).data(0x14000023u32);

    // Forces the mining tool used to be Pickaxe
    skyline::patching::Patch::in_text(0xf0f820).data(0x321F03E1u32);

    // Related to Crafting Table auto-respawn
    // Removes the link event setting the auto respawn timer
    skyline::patching::Patch::in_text(0xf0c20c).data(0x140003ACu32);
    // Disables the count_down_int for the auto respawn timer
    skyline::patching::Patch::in_text(0xf088c8).nop();
    // Skips the auto respawn create table call
    skyline::patching::Patch::in_text(0xf088cc).data(0x14000005u32);
}