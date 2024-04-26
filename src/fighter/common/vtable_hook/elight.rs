pub fn install() {
    // Patches which statuses are checked for Foresight
    skyline::patching::Patch::in_text(0xa2951c).data(0x7100111Fu32);
    skyline::patching::Patch::in_text(0xa29594).data(0x7100111Fu32);
    skyline::patching::Patch::in_text(0xa29008).data(0x71000F5Fu32);
}