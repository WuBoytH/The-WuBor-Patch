pub fn install() {
    skyline::patching::Patch::in_text(0x60eb08).data(0x52800001u32);
}