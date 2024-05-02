pub fn install() {
    // Disables Foresight
    skyline::patching::Patch::in_text(0xa28e78).nop();
    skyline::patching::Patch::in_text(0xa28e84).data(0x140000ACu32);
}