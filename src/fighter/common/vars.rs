use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::smashball
    },
    smashline::*,
    wubor_utils::vars::*,
    crate::fighter::donkey::vars::*
};

#[fighter_reset]
fn fighter_reset(_fighter: &mut L2CFighterCommon) {
    unsafe {
        DK_COUNT = 0;
        if !smashball::is_training_mode() {
            FGC_TRAINING = false;
        }
    }
}

pub fn install() {
    install_agent_resets!(
        fighter_reset
    );
}