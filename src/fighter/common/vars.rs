use {
    smash::lua2cpp::L2CFighterCommon,
    smashline::*,
    wubor_utils::vars::*,
};

#[fighter_reset]
fn fighter_reset(_fighter: &mut L2CFighterCommon) {
    unsafe {
        FGC_TRAINING = false;
    }
}

pub fn install() {
    install_agent_resets!(
        fighter_reset
    );
}