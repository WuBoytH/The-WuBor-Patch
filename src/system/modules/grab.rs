use crate::offsets;

pub fn install() {
    unsafe {
        // Disables the LR check
        let _ = skyline::patching::Patch::in_text(offsets::system::grab_module::LR_CHECK_PATCH).nop();
    }
}
