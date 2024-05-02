pub fn install() {
    // Pill Fix for respawn platform
    let _ = skyline::patching::Patch::in_text(0xcc9e34).data(0x14000047u32);
}