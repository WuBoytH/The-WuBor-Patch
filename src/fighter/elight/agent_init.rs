use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_cancel::*,
    crate::fighter::element::fgc::*
};

#[event(start)]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_ELIGHT {
            return;
        }
        let agent = (*fighter.battle_object).agent_kind_hash;
        element_fgc(agent);
    }
}

pub fn install() {
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_elight"));
    agent_reset::install();
}
