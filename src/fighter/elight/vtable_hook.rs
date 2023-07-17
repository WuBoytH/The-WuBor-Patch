pub fn install() {
    // Patches which statuses are checked for Foresight
    skyline::patching::Patch::in_text(0xa294fc).data(0x71000D1Fu32);
    skyline::patching::Patch::in_text(0xa29574).data(0x71000D1Fu32);
    skyline::patching::Patch::in_text(0xa28fe8).data(0x71000F5Fu32);
}