pub fn install() {
    // Always start a match with Levin Sword
    skyline::patching::Patch::in_text(0x1005d30).nop();
}