use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_cancel::*,
    super::fgc::*
};

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if ![*FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT].contains(&fighter_kind) {
            return;
        }
        let agent = (*fighter.battle_object).agent_kind_hash;
        element_fgc(agent);
    }
}

pub fn install() {
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_eflame"));
    CustomCancelManager::initialize_agent(Hash40::new("fighter_kind_elight"));
    install_agent_init_callbacks!(
        agent_init
    );
}
