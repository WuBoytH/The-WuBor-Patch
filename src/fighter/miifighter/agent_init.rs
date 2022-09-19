use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::*,
        phx::*,
        lib::lua_const::*
    },
    custom_cancel::*,
    smashline::*,
    super::fgc
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_MIIFIGHTER {
            return;
        }
        fgc::install();
    }
}

pub fn install() {
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_miifighter"));
    install_agent_resets!(
        agent_reset
    );
}
