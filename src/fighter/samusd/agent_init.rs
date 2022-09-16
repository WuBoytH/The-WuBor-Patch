use {
    smash::{
        lua2cpp::L2CFighterCommon,
        phx::*,
        app::*,
        lib::lua_const::*
    },
    smashline::*,
    custom_var::*,
    custom_cancel::*,
    wubor_utils::vars::*,
    super::fgc
};

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_SAMUSD {
            return;
        }
        VarModule::set_int(fighter.battle_object, samusd::instance::int::CSHOT_ID, *BATTLE_OBJECT_ID_INVALID);
        fgc::install();
    }
}

pub fn install() {
    let agent = Hash40::new("fighter_kind_samusd");
    CustomCancelManager::initialize_agent(agent);
    install_agent_resets!(
        agent_reset
    );
}
