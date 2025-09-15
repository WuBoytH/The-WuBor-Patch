pub fn install() {
    // Always start a match with Levin Sword
    let _ = skyline::patching::Patch::in_text(0x1005d30).nop();

    // Fixes an issue causing Robin to be un-trippable after using Thoron
    let _ = skyline::patching::Patch::in_text(0x1009970).nop();
}