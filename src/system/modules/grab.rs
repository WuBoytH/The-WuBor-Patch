pub fn install() {
    // Disables the LR check
    let _ = skyline::patching::Patch::in_text(0x45c85c).nop();
}
