use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    wubor_utils::vars::*
};

#[fighter_reset]
fn fighter_reset(_fighter: &mut L2CFighterCommon) {
    unsafe {
        donkey::DK_COUNT = 0;
    }
}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_DONKEY {
            return;
        }
        donkey::DK_COUNT += 1;
    }
}

pub fn install() {
    install_agent_resets!(
        fighter_reset
    );
    install_agent_init_callbacks!(
        agent_init
    );
}
